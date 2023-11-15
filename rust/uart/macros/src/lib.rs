extern crate proc_macro2;
extern crate syn;
extern crate quote;
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(GetFn)]
pub fn derive_getter_fn(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(_item as DeriveInput);
    eprintln!("{:#?}", ast);
    let name = &ast.ident;
    let var_names : syn::DataStruct = match ast.data {
        syn::Data::Struct(c) => c,
        _ => unimplemented!()
    };

    eprintln!("{:#?}", var_names);
    eprintln!("#######################");
    let named_fields = match var_names.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            named
        }
        _ => unimplemented!()
    };
    // eprintln!("{:#?}", named_fields);
    let func_names = named_fields.iter().map(|f| {
        // let ident = &f.ident;
        let ident = Ident::new(&format!("get_{}", f.ident.as_ref().unwrap()), Span::call_site());
        quote!{ #ident }
    });
    let reg_names = named_fields.iter().map(|f| {
        let ident = &f.ident;
        quote!{ #ident }
    });
    // eprintln!("{:#?}", x);
    // for x in named_fields {
    //     let y = match x.ident {
    //         Some(I) => eprintln!("{}",I),
    //         None => unimplemented!()
    //     };
    // } 
    let expanded = quote! {
        impl #name {
            #(pub fn #func_names (&self) -> u8{ return #reg_names.read() })*
        } 
    };
    eprintln!("Expanded");
    eprintln!("{:#?}", expanded);

    proc_macro::TokenStream::from(expanded)
    
}
