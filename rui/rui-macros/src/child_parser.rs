use darling::FromMeta;
use kdl::KdlNode;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, TypePath};

use crate::{
    CompilerContext, STD_WIDGETS, UnimplementedError, VariableDefaultInfo, entry_to_expression,
    sanitize_widget_name,
};

pub fn build_child_struct(
    child: &KdlNode,
    mut context: CompilerContext,
) -> miette::Result<(Ident, proc_macro2::TokenStream, CompilerContext)> {
    let short_widget_type = sanitize_widget_name(&child.name().to_string());
    let overrides = child
        .iter_children()
        .map(|c| {
            (
                Ident::new(&c.name().to_string(), Span::call_site()),
                entry_to_expression(c.entries().first().ok_or(UnimplementedError {}).unwrap())
                    // lmao
                    .unwrap()
                    .ok_or(UnimplementedError {})
                    .unwrap(),
            )
        })
        .map(|(name, default)| VariableDefaultInfo { name, default })
        .collect::<Vec<VariableDefaultInfo>>();

    let mut ephemeral_data_info = None;
    // disgusting pointers. seriously what
    let widget_type_string = if STD_WIDGETS
        .iter()
        .find(|w| *w == &short_widget_type)
        .is_some()
    {
        // Also handle ephemeral data here.
        ephemeral_data_info = Some(
            TypePath::from_string(&format!("rui::widgets::{}Ephemeral", &short_widget_type))
                .map_err(|_| UnimplementedError {})?,
        );
        format!("rui::widgets::{}", &short_widget_type)
    } else {
        todo!()
        // Widget type must then be a file path to a widget file
        // Check if we've already compiled a general struct for this widget. If not, compile one.
        // TODO

        // format!("Auto_{}_General", sanitize_widget_name(&widget_type),)
    };
    let widget_type = TypePath::from_string(&widget_type_string).unwrap();

    context.struct_id += 1;
    let wrapper_name = format!("Auto_{}_{}", &short_widget_type, context.struct_id);
    let wrapper_id = Ident::new(&wrapper_name, Span::call_site());

    let mut out_tokens = TokenStream::new();
    // Now, build a wrapper struct with the additional children.
    // TODO: Track + add children - make this cleaner.
    if let Some(ephemeral_ident) = ephemeral_data_info {
        out_tokens.extend(quote! {
            #[allow(non_camel_case_types)]
            struct #wrapper_id {
                inner_widget: #widget_type,
                ephemeral: Option<#ephemeral_ident>,
            }

            impl rui::Widget<()> for #wrapper_id {
                fn render(
                    &mut self,
                    render_pass: &mut rui::wgpu::RenderPass,
                    graphics_state: &rui::AppGraphicsState,
                    _ephemeral: &(),
                ) {
                    if let Some(e) = &self.ephemeral {
                        self.inner_widget.render(render_pass, graphics_state, e);
                    } else {
                        let e = self.inner_widget.update_ephemeral(graphics_state, None);
                        self.inner_widget.render(render_pass, graphics_state, &e);
                        self.ephemeral = Some(e);
                    }
                }

                fn destroy(&mut self) {
                    if let Some(e) = &mut self.ephemeral {
                        e.destroy();
                        drop(e);
                        self.ephemeral = None;
                    }
                    self.inner_widget.destroy();
                }

                fn update_ephemeral(
                    &self,
                    _graphics_state: &rui::AppGraphicsState,
                    _old: Option<()>,
                ) -> () {}
            }

            impl Default for #wrapper_id {
                fn default() -> Self {
                    Self {
                        inner_widget: #widget_type {
                            #(#overrides)*
                            ..Default::default()
                        },
                        ephemeral: None,
                    }
                }
            }

        });
    } else {
        out_tokens.extend(quote! {
            #[allow(non_camel_case_types)]
            struct #wrapper_id {
                inner_widget: #widget_type,
                // No ephemeral data
            }

            impl rui::Widget<()> for #wrapper_id {
                fn render(
                    &mut self,
                    render_pass: &mut rui::wgpu::RenderPass,
                    graphics_state: &rui::AppGraphicsState,
                    _ephemeral: &(),
                ) {
                    self.inner_widget.render(render_pass, graphics_state, ());
                }

                fn destroy(&mut self) {
                    self.inner_widget.destroy();
                }

                fn update_ephemeral(
                    &self,
                    _graphics_state: &rui::AppGraphicsState,
                    _old: Option<()>,
                ) -> () {}
            }

            impl Default for #wrapper_id {
                fn default() -> Self {
                    Self {
                        inner_widget: #widget_type {
                            #(#overrides)*
                            ..Default::default()
                        },
                    }
                }
            }

        });
    }

    Ok((wrapper_id, out_tokens, context))
}
