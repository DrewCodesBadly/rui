use std::{fs::File, io::Read};

use kdl::{KdlDocument, KdlEntry};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Ident, LitStr, Token, parse::Parse, parse_macro_input};

use crate::errors::{StructureErrors, UnimplementedError};

mod errors;

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
    let mut output = build_state_struct(
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
            // TODO: Auto-generate these methods
            fn render(&mut self) {

            }

            fn handle_event(&mut self, event: rui::AppEvent) {

            }
        }
    });

    output.into()
}

fn read_ui_file(main_file: &str) -> miette::Result<KdlDocument> {
    let mut file =
        File::open(&main_file).expect(&format!("Unable to locate file \"{}\"", main_file));
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect(&format!("Cannot read contents of file \"{}\"", main_file));
    Ok(s.parse()?)
}

struct VariableInfo {
    var_type: String,
    name: String,
    default: String,
}

impl ToTokens for VariableInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let var_type = &self.var_type;
        tokens.extend(quote!(#name: #var_type,));
    }
}

struct VariableDefaultInfo {
    name: String,
    default: String,
}

impl ToTokens for VariableDefaultInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let def = &self.default;
        tokens.extend(quote!(#name: #def,));
    }
}

fn build_state_struct(
    kdl: KdlDocument,
    mut context: CompilerContext,
    struct_ident: Ident,
) -> miette::Result<proc_macro2::TokenStream> {
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
                var_type: t.clone(),
                name: entry.name().ok_or(UnimplementedError {})?.to_string(),
                default: entry_to_expression(entry)?.unwrap_or(String::from("Default::default()")),
            });
        }
    }

    // Begin parsing child widgets and adding them to the struct
    for (i, child) in kdl.nodes().iter().skip(1).enumerate() {
        let short_widget_type = child.name().to_string();
        let overrides = child
            .iter_children()
            .map(|c| {
                (
                    child.name().to_string(),
                    entry_to_expression(c.entries().first().ok_or(UnimplementedError {}).unwrap())
                        // lmao
                        .unwrap()
                        .ok_or(UnimplementedError {})
                        .unwrap(),
                )
            })
            .collect::<Vec<(String, String)>>();
        // disgusting pointers. seriously what
        let widget_type = if STD_WIDGETS
            .iter()
            .find(|w| *w == &short_widget_type)
            .is_some()
        {
            format!("rui::{}", &short_widget_type)
        } else {
            panic!("unimplemented!!!");
            // Widget type must then be a file path to a widget file
            // Check if we've already compiled a general struct for this widget. If not, compile one.
            // TODO

            // format!("Auto_{}_General", sanitize_widget_name(&widget_type),)
        };

        context.struct_id += 1;
        let wrapper_id = Ident::new(
            &format!("Auto_{}_{}", &short_widget_type, context.struct_id),
            Span::call_site(),
        );

        let default_struct = format!(
            "Self {{\n {} }}",
            overrides
                .iter()
                .fold(format!("{} {{\n", &widget_type), |acc, o| {
                    format!("{}{}: {},\n", acc, o.0, o.1)
                })
                + "..Default::default()\n}"
        );

        // Now, build a wrapper struct with the additional children.
        // TODO: Track + add children
        out_tokens.extend(quote! {
            #[allow(non_camel_case_types)]
            struct #wrapper_id {
                inner_widget: #widget_type,
            }

            impl rui::Widget for #wrapper_id {
                fn render(&self) {
                    self.inner_widget.render();
                }
            }

            impl Default for #wrapper_id {
                fn default() -> Self {
                    #default_struct
                }
            }

        });
        vars.push(VariableInfo {
            name: format!("child_{}", i),
            default: format!("{}::default()", &widget_type),
            var_type: widget_type,
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

        // TODO
        impl rui::Widget for #struct_ident {
            fn render(&self) {

            }
        }
    });

    Ok(out_tokens)
}

fn entry_to_expression(entry: &KdlEntry) -> miette::Result<Option<String>> {
    if entry.ty().filter(|i| i.to_string() == "expr").is_some() {
        // TODO: parse expression for errors, if at all possible
        Ok(entry.value().as_string().map(|s| s.to_owned()))
    } else {
        Ok(match entry.value() {
            kdl::KdlValue::String(s) => Some(format!("String::from({})", s)),
            kdl::KdlValue::Integer(i) => Some(format!("{}", i)),
            kdl::KdlValue::Float(f) => Some(format!("{}", f)),
            kdl::KdlValue::Bool(b) => Some(format!("{}", b)),
            kdl::KdlValue::Null => None,
        })
    }
}

fn sanitize_widget_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
