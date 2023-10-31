extern crate proc_macro;
use proc_macro::{TokenStream, Ident};
use quote::quote;
use syn::{FieldsNamed, Attribute, Data, DataStruct, DeriveInput, Field, Fields, ItemStruct, parse_macro_input, Meta, Path, PathSegment};

#[proc_macro_derive(Schema, attributes(owned_by))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = extract_fields_from_schema(input);
    if let Some(fields) = fields {
        let field = find_fk_field(&fields).unwrap();
        println!("{:?}", field);
        let owner_type_ident = {
            let owner_type = find_owner(&field).unwrap();
            syn::Ident::new(&owner_type, proc_macro2::Span::call_site())
        };
        // get the posted_by id
        let owner_id = {
            
        };
        let gen = quote! {
            impl SafeDelete for #owner_type_ident {
                fn safe_delete(&self) {
                    println!("Safely deleting");
                    println!("Hello my id is {:?}", self.id);
                }
            }
        };
        return gen.into()
    }

    TokenStream::new()
}

fn extract_fields_from_schema(input: DeriveInput) -> Option<FieldsNamed> {
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            Some(fields)
        } else {
            // compiler error
            None
        }
    } else {
        None
    }
}

fn find_fk_field(fields: &FieldsNamed) -> Option<&Field>{
    for field in fields.named.iter() {
        if field.attrs.len() > 0 {
            for attr in &field.attrs {
                if let Meta::List(ml) = &attr.meta {
                    for seg in &ml.path.segments {
                        if seg.ident.to_string() == "owned_by".to_string() {
                            return Some(field);
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn find_owner(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        if let Meta::List(ml) = &attr.meta {
            return Some(ml.tokens.to_string());
        }
    }
    None
}
// TODO: derive macro that implements functions for deletion and access
// TODO: think about how to link user struct and post struct
// TODO: start with issuing deletion function on a single struct
// OR
// info being passed to the macro gets stored in a data structure somewhere
//
//
// Application calls delete on User struct (gives it user key) -> macro figures out what else 
// depends on the user struct and recurses and so would have to provide the foreign key
//
// macro takes in additional parameters that handles which fields refer to which collection
// simplify to OWNED_BY for now and then OWNS as a long time goal
// think about shared data
