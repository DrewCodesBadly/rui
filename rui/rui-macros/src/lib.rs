use std::{fs::File, io::Read, path::PathBuf};

use kdl::KdlDocument;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Token, parse::Parse, parse_macro_input};

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

    let main_file_parsed = read_ui_file(main_file);

    let output = quote! {
        struct #struct_name {
            global_state: #global_state_type,
            graphics_state: rui::AppGraphicsState,
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
    };

    output.into()
}

fn read_ui_file(main_file: String) -> miette::Result<KdlDocument> {
    let mut file =
        File::open(&main_file).expect(&format!("Unable to locate file \"{}\"", main_file,));
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect(&format!("Cannot read contents of file \"{}\"", main_file));
    Ok(s.parse()?)
}
