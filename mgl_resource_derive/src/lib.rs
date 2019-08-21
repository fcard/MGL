//! Automatically generate a parsing method for a given resource.
//!
//! e.g.
//!   #[derive(Resource)]
//!   struct Sprite {
//!     field: i64,
//!     other_field: String,
//!   }
//!
//! Now we can do the following:
//!   use crate::parser::parse_code;
//!   use crate::resources::resource_trait::ResourceCreate;
//!
//!   let ast = parse_code(
//!   r"""
//!     sprite s {
//!       field: 12
//!       other_field: "abc"
//!     }
//!   """);
//!
//!   let sprite = Sprite::new(&ast);
//!   println!("{}", sprite.field)       // 12
//!   println!("{}", sprite.other_field) // 1.0
//!
#![feature(box_patterns)]

extern crate proc_macro;

use proc_macro2::{TokenStream, Span};
use syn::*;
use quote::*;

macro_rules! ident {
  ($name: ident) => { Ident::new(stringify!($name), Span::call_site()) }
}

#[proc_macro_derive(Resource, attributes(array_field, sub_resource, ignore_field))]
pub fn derive_resource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name   = input.ident;
  let fields = named_fields(&input.data);
  let parse  = impl_parse_key_values(&fields);

  proc_macro::TokenStream::from(quote! {
    impl<T: ResourceAst> Resource<T> for #name {
      #parse
    }
  })
}

fn named_fields(data: &Data) -> FieldsNamed {
  match data {
    &Data::Struct(ref data) => {
      match data.fields {
        Fields::Named(ref fields) => {
          fields.clone()
        }
        _ => panic!("Only structs with named fields are allowed!")
      }
    }
    _ => panic!("Only structs are allowed!")
  }
}

fn impl_parse_key_values(fields: &FieldsNamed) -> TokenStream {
  let mut matches = Vec::new();

  // Variable Names (to avoid misspellings by making them compile errors)
  let source_ast    = quote! { source_ast };
  let key           = quote! { key };
  let value         = quote! { value };
  let array_index   = quote! { array_index };
  let sub_field_key = quote! { sub_field_key };
  let module        = quote! { crate::resources::resource_trait };

  for field in &fields.named {
    // Field inspection

    let field_name  = &field.ident.clone();
    let field_str   = field_str(&field);
    let field_attrs = field_attributes(&field);
    let field_sub   = field_attrs.sub;
    let field_array = field_attrs.array;

    // Variables that will hold the final code

    let field_set;
    let array_pre_code;
    let field_indexing;
    let no_field_assert;

    // Handling array fields

    if field_array {
      array_pre_code = quote! {
        let #array_index = #module::KeyInspector::get_array_index(#field_str, #key.clone())?;
        if #array_index >= self.#field_name.len() {
          self.#field_name.resize_with(#array_index + 1, Default::default);
        }
      };
      field_indexing = quote! { [#array_index] };

    } else {
      array_pre_code = quote! {};
      field_indexing = quote! {};
    }

    let full_field = quote! { #field_name#field_indexing };

    // Handling subfields

    if field_sub {
      field_set = quote! {
        let #sub_field_key = #module::KeyInspector::get_sub_field_key(#field_str, #key)?;
        self.#full_field.parse_key_value(#source_ast, #sub_field_key, #value)?;
      }

    } else {
      field_set = quote! {
        self.#full_field = #module::parse_field_default(#key, #value)?;
      }
    }

    // Assert that field is simple if that's what is expected

    if field_array || field_sub {
      no_field_assert = quote! {};

    } else {
      no_field_assert = quote! {
        #module::KeyInspector::assert_field_has_no_index(#field_str, #key.clone())?;
      };
    }

    // Add Match

    if !field_attrs.ignore {
      matches.push(quote! { #field_str => {
        #array_pre_code
        #no_field_assert
        #field_set
      }});
    }
  }

  // Assemble Method

  quote! {
    fn parse_key_value(&mut self, #source_ast: &T, #key: Key, #value: Expression) -> #module::Result<()> {
      match #key.name_of().as_ref() {
        #(#matches),*,
        field => {
          return #module::MglError::invalid_field(field, #module::InvalidFieldKind::NotFound)
        }
      }
      Ok(())
    }
  }
}

fn field_str(field: &Field) -> String {
  field.ident.as_ref().map(Ident::to_string).unwrap_or(String::new())
}

struct FieldAttributes {
  sub:   bool,
  array: bool,
  ignore: bool,
}

fn field_attributes(field: &Field) -> FieldAttributes {
  let mut attributes = FieldAttributes { sub: false, array: false, ignore: false };

  for attr in field.attrs.clone() {
    if attr.path.is_ident(&ident!(sub_resource)) {
      attributes.sub = true;

    } else if attr.path.is_ident(&ident!(array_field)) {
      attributes.array = true;

    } else if attr.path.is_ident(&ident!(ignore_field)) {
      attributes.ignore = true;

    }
  }
  attributes
}

