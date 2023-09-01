extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Usage:
/// ```
/// use proc_non_empty_string::non_empty_string;
/// use non_empty_string::NonEmptyString;
///
/// let value = non_empty_string!("example");
/// dbg!(value);
#[proc_macro]
pub fn non_empty_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let value = input.value();
    if value.is_empty() {
        quote! {
            compile_error!("string is empty")
        }
    } else {
        quote! {
            NonEmptyString::new(#input.to_string()).unwrap()
        }
    }
    .into()
}
