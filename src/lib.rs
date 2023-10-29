extern crate proc_macro;
use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Field, Fields, ItemStruct};

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
