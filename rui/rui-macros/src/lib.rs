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
    let output = quote! {
        struct #struct_name {
            global_state: #global_state_type,
            graphics_state: rui::AppGraphicsState,
        }

        impl #struct_name {
            pub async fn new(
                display_handle: rui::raw_window_handle::RawDisplayHandle,
                window_handle: rui::raw_window_handle::RawWindowHandle
            ) -> Result<Self, rui::AppStateCreationError> {
                Ok(Self {
                    global_state: #global_state_type::default(),
                    graphics_state: rui::AppGraphicsState::new(display_handle, window_handle).await?,
                })
            }
        }
    };

    output.into()
}
