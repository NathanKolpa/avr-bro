use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse::Error as ParseError, parse_macro_input, spanned::Spanned, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn avr_main(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);

    let item_ident = &item_fn.sig.ident;

    if !item_fn.sig.inputs.is_empty() {
        return ParseError::new(
            item_fn.sig.inputs.span(),
            "#[avr_main] Cannot have arguments",
        )
        .to_compile_error()
        .into();
    }

    let returns_never = match item_fn.sig.output {
        ReturnType::Default => false,
        ReturnType::Type(_, ref return_type) => matches!(**return_type, syn::Type::Never(_)),
    };

    let exit_guard = if !returns_never {
        quote! { loop {} }
    } else {
        quote!()
    };

    let quote = quote! {
        #[doc(hidden)]
        #[export_name = "main"]
        pub extern "C" fn entry_point() {
            #item_ident();
            #exit_guard
        }

        #item_fn
    };

    quote.into()
}
