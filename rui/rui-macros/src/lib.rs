use std::{fs::File, io::Read};

use darling::FromMeta;
use kdl::{KdlDocument, KdlEntry};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, Ident, LitStr, Token, TypePath, parse::Parse, parse_macro_input, token};

use crate::{
    child_parser::build_child_struct,
    errors::{StructureErrors, UnimplementedError},
    token_help::{VariableDefaultInfo, VariableInfo, VariableType, entry_to_expression},
};

mod child_parser;
mod errors;
mod token_help;

// TODO: maybe a better solution
const STD_WIDGETS: [&str; 1] = ["Rectangle"];

struct GenerateAppStateOpts {
    main_file: String,
    global_state_type: Ident,
    struct_name: Ident,
}

impl Parse for GenerateAppStateOpts {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let main_file = input.parse::<LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let global_state_type = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let struct_name = input.parse::<Ident>()?;
        Ok(Self {
            main_file,
            global_state_type,
            struct_name,
        })
    }
}

// Struct passed around while the compiler does its thing. Hangs on to useful information.
struct CompilerContext {
    file_path: String,
    struct_id: u64,
}

fn read_ui_file(main_file: &str) -> miette::Result<KdlDocument> {
    let mut file =
        File::open(&main_file).expect(&format!("Unable to locate file \"{}\"", main_file));
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect(&format!("Cannot read contents of file \"{}\"", main_file));
    Ok(s.parse()?)
}

/// This macro compiles your app into rust code by generating a struct representing the app's state.
/// You need to provide the relative path to the main file of the app, the type of the global state struct,
/// and the name of the generated app state struct.
#[proc_macro]
pub fn generate_app_state(item: TokenStream) -> TokenStream {
    let GenerateAppStateOpts {
        main_file,
        global_state_type,
        struct_name,
    } = parse_macro_input!(item as GenerateAppStateOpts);

    let main_file_parsed = read_ui_file(&main_file).unwrap();

    // TODO: Build general structs for all widget files, and ensure that there are no circular dependencies.

    // Begin creating the app state recursively.
    let (mut output, _context) = build_state_struct(
        main_file_parsed,
        CompilerContext {
            file_path: main_file,
            struct_id: 0,
        },
        Ident::new("Auto_MainWidget_0", Span::call_site()),
    )
    .unwrap();

    output.extend(quote! {
        struct #struct_name {
            global_state: #global_state_type,
            graphics_state: rui::AppGraphicsState,
            root_widget: Auto_MainWidget_0,
        }

        impl #struct_name {
            async fn new(
                window_handle: impl Into<rui::wgpu::SurfaceTarget<'static>>,
                width: u32,
                height: u32,
            ) -> Result<Self, rui::AppStateCreationError> {
                Ok(Self {
                    global_state: #global_state_type::default(),
                    graphics_state: rui::AppGraphicsState::new(window_handle, width, height).await?,
                    root_widget: Auto_MainWidget_0::default(),
                })
            }
        }

        impl rui::AppState for #struct_name {
            fn render(&mut self) {
                // Fails silently (shouldn't fail, anyway - was probably a fluke if it didn't.)
                let _ = self.graphics_state.start_render(&mut self.root_widget);
            }

            fn handle_event(&mut self, event: rui::AppEvent) {

            }
        }
    });

    output.into()
}

fn build_state_struct(
    kdl: KdlDocument,
    mut context: CompilerContext,
    struct_ident: Ident,
) -> miette::Result<(proc_macro2::TokenStream, CompilerContext)> {
    let state_node = kdl
        .nodes()
        .first()
        .ok_or(StructureErrors::MissingState(context.file_path.clone()))?;

    let mut out_tokens = quote! {};

    // Get a list of all the variable info in the state
    let mut vars = Vec::<VariableInfo>::new();
    for child in state_node.iter_children() {
        let t = child.name().to_string();
        for entry in child.entries() {
            vars.push(VariableInfo {
                var_type: VariableType::from(&t),
                name: Ident::new(
                    &entry.name().ok_or(UnimplementedError {})?.to_string(),
                    Span::call_site(),
                ),
                default: entry_to_expression(entry)?
                    .unwrap_or(Expr::from_string("Default::default()").unwrap()),
            });
        }
    }

    // Begin parsing child widgets and adding them to the struct
    let mut child_widgets = Vec::<Ident>::new();
    for (i, child) in kdl.nodes().iter().skip(1).enumerate() {
        let (child_ident, tokens, new_context) = build_child_struct(child, context)?;
        context = new_context;
        out_tokens.extend(tokens);
        let name = Ident::new(&format!("child_{}", i), Span::call_site());
        child_widgets.push(name.clone());
        vars.push(VariableInfo {
            name,
            default: Expr::from_string(&format!("{}::default()", &child_ident.to_string()))
                .map_err(|_| UnimplementedError {})?,
            var_type: VariableType::SynIdent(child_ident),
        });
    }

    let vars_defaults = vars.iter().map(|v| VariableDefaultInfo {
        name: v.name.clone(),
        default: v.default.clone(),
    });

    out_tokens.extend(quote! {
        #[allow(non_camel_case_types)]
        struct #struct_ident {
            #(#vars)*
        }

        impl Default for #struct_ident {
            fn default() -> Self {
                Self {
                    #(#vars_defaults)*
                }
            }
        }

        impl rui::Widget<()> for #struct_ident {
            fn render(
                &mut self,
                render_pass: &mut rui::wgpu::RenderPass,
                graphics_state: &rui::AppGraphicsState,
                _ephemeral: &(),
            ) {
                #(self.#child_widgets.render(render_pass, graphics_state, &()))*
            }

            fn destroy(&mut self) {
                #(self.#child_widgets.destroy())*
            }

            fn update_ephemeral(
                &self,
                _graphics_state: &rui::AppGraphicsState,
                _old: Option<()>,
            ) -> () {}

        }
    });

    Ok((out_tokens, context))
}

fn sanitize_widget_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
