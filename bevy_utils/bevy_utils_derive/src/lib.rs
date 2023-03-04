extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(UtilEvent)]
pub fn event(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;
    // Build the impl
    let gen = quote! {
        impl UtilEvent for #name {}
    };

    // Return the generated impl
    gen.parse().unwrap()
}
