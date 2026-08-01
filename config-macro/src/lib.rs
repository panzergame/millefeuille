use proc_macro::TokenStream;
use quote::format_ident;
use syn::{Ident, parse_macro_input};

fn do_generate_accessors(fields: &syn::FieldsNamed, class_name: &Ident) -> Result<TokenStream, String> {
    let methods: Result<Vec<_>, _> = fields
        .named
        .iter()
        .map(|field| -> Result<_, String>{
            let name = field.ident.as_ref().ok_or("missing field name")?;
            let ty = &field.ty;
            let setter = format_ident!("set_{}", name);
            Ok(quote::quote! {
                pub fn #setter(&mut self, value: #ty) {
                  self.#name = value;
                  self.save().expect("Failed to save config");
                }
                
                pub fn #name(&self) -> #ty {
                  self.#name
                }
            })
        })
        .collect();
    let methods = methods?;
    println!("class name {}", class_name);
    Ok(TokenStream::from(quote::quote! {
      impl #class_name {
        #(#methods)*
      }
    }))
}

#[proc_macro_derive(ConfigHelper)]
pub fn generate_accessors(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    let class_name = input.ident;
    if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            if let Ok(generate_tokens) = do_generate_accessors(fields, &class_name) {
              println!("{}", generate_tokens);
              return generate_tokens;
            }
        }
    }

    TokenStream::new()
}
