extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Field, Fields, ItemStruct};

// #[proc_macro]
// pub fn hello_world_m(_item: TokenStream) -> TokenStream {
//     "println!(\"hello world\")".parse().unwrap()
// }

#[proc_macro_attribute]
pub fn owned_by(attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", input.to_string());
    input
    // let input = parse_macro_input!(input as ItemStruct);
    // for attr in input.attrs {
    //     println!("attr:, {:?}", stringify!(attr));
    // }
    // "println(\"owner is me\")".parse().unwrap()
}
// if let Data::Struct(DataStruct { fields, .. }) = &input.fields {
//     let struct_name = &input.ident;

// let mut generated_code = quote! {};
// generated_code = quote! {
//     #generate_code

// }
// for field in fields.iter() {
//     let field_name = &field.ident;
//     let field_type = &field.ty;

//     generated_code = quote! {
//         #generated_code
//         impl #struct_name {
//             pub fn #field_name(&self) -> Option<String> {
//                 Some(format!("The field {} is owned by {}.", stringify!(#field_name), stringify!(#field_type)))
//             }
//         }
//     };
// }

// return generated_code.into();
// }

// TokenStream::new()
// }
// }

fn field_has_owned_by_attribute(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs {
        if let Ok(meta) = attr.parse_args() {
            if let syn::Meta::Path(path) = meta {
                if path.is_ident("OWNED_BY") {
                    return true;
                }
            }
        }
    }
    false
}
