extern crate proc_macro;
extern crate syn;
extern crate quote;
use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(GetFn)]
pub fn derive_getter_fn(_item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(_item as DeriveInput);
    eprintln!("{:#?}", ast);
    let name = &ast.ident;
    let var_names:syn::DataStruct = match ast.data {
        syn::Data::Struct(c) => c,
        _ => unimplemented!()
    };

    eprintln!("{:#?}", var_names);
    eprintln!("#######################");

    for x in var_names.fields.iter() {
        eprintln!("{:#?}", x.ident);
    }
    let expanded = quote! {
        impl #name {
        } 
    };

    TokenStream::from(expanded)
    
}
