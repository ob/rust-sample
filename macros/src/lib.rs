extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sample_macro(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    // Uncomment this line to make VS Code freeze
    // println!("input = {}", input);
    let output = quote! {
        println!("input = {}", #input);
    };
    output.into()
}
