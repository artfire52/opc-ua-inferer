use proc_macro::TokenStream;
use syn::{DataStruct,Data,Fields, DeriveInput, DataEnum, Field, token::{Comma}, Ident, Variant, GenericParam};
use quote::quote;

///derive the Serialize trait
#[proc_macro_derive(Serialize)]
 pub fn derive_serialize(input: TokenStream) -> TokenStream { 
   let ast = syn::parse_macro_input!(input as DeriveInput);
    impl_serialize_macro(&ast)

 }

fn impl_serialize_macro(ast: &syn::DeriveInput)-> TokenStream{
   let generic= &ast.generics.params.first();
   let name = &ast.ident;
   match &ast.data {
      Data::Struct( DataStruct{fields: Fields::Named(fields),..})=>
         impl_serialize_macro_struct(&fields.named,generic,name),
      Data::Enum( DataEnum{variants: v,..})=>
         impl_serialize_macro_enum(&v,generic,name),
      _=> panic!("struct need to have named fields") }
 }

 fn impl_serialize_macro_struct(fields: &syn::punctuated::Punctuated<Field, Comma>,generic:&Option<&GenericParam>,struct_name: &Ident) -> TokenStream {   
   let field_name = fields.iter().map(|field| &field.ident);
   match generic {
       Some(GenericParam::Lifetime(_))=>{
         let expanded = quote! {
            impl<'a> Serialize for #struct_name<'a>{
                  fn serialize(&self)->Vec<u8>{
                     let mut result: Vec<u8> =Vec::new();
                     #(
                        result.extend_from_slice(&self.#field_name.serialize());
                     )*
                     result
                  }
               }

            };
            TokenStream::from(expanded)

       },
       Some(GenericParam::Type(_))=>{
         let expanded = quote! {
            impl<T:Serialize+Deserialize> Serialize for #struct_name<T>{
                  fn serialize(&self)->Vec<u8>{
                     let mut result: Vec<u8> =Vec::new();
                     #(
                        result.extend_from_slice(&self.#field_name.serialize());
                     )*
                     result
                  }
               }

            };
            TokenStream::from(expanded)

       },
       _=>{
         let expanded = quote! {
       impl Serialize for #struct_name{
             fn serialize(&self)->Vec<u8>{
                let mut result: Vec<u8> =Vec::new();
                #(
                   result.extend_from_slice(&self.#field_name.serialize());
                )*
                result
             }
          }

       };
       TokenStream::from(expanded)
    },
   }
    
}


fn impl_serialize_macro_enum(variants: &syn::punctuated::Punctuated<Variant, Comma>,generic:&Option<&GenericParam>,struct_name: &Ident) -> TokenStream {   
   let field_name = variants.iter().map(|field| &field.ident);
   match generic {
      Some(GenericParam::Lifetime(_))=>{
      let expanded = quote! {
         impl<'a> Serialize for #struct_name<'a>{
            fn serialize(&self)->Vec<u8> {
               match self {
                  #(
                     #struct_name::#field_name(s)=>s.serialize(),
                  )*
                  }
            }
            }
   
            // ...
         };
         TokenStream::from(expanded)
   
      },
      _=>{
         let expanded = quote! {
            impl Serialize for #struct_name {
               fn serialize(&self)->Vec<u8> {
                  match self {
                     #(
                        #struct_name::#field_name(s)=>s.serialize(),
                     )*
   
                     }
               }
            }
   
         };
         TokenStream::from(expanded)
      },
   }
       
}
 


///derive the Deserialize trait
 #[proc_macro_derive(Deserialize)]
 pub fn derive_deserialize(input: TokenStream) -> TokenStream { 
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    // Build the trait implementation
    impl_deserialize_macro(&ast)
 }

fn impl_deserialize_macro(ast: &syn::DeriveInput) -> TokenStream {
   let fields= match &ast.data {Data::Struct( DataStruct{fields: Fields::Named(fields),..})=>&fields.named,
   _=> panic!("struct need to have fields") };
   let field_name: Vec<_> = fields.iter().map(|field| &field.ident).collect();
   let field_type = fields.iter().map(|field| &field.ty);
   let struct_name = &ast.ident;
   let struct_lifetime=&ast.generics.params.first();
   match struct_lifetime {
   
   Some(GenericParam::Lifetime(_))=>{
      let expanded = quote! {
         impl<'a> Deserialize for #struct_name<'a>{
            fn deserialize(data:&[u8])-> MapperResult<(&[u8], Self)>{
                  #(
                  let (data,#field_name)=#field_type::deserialize(data)?;
                  )*
                  let return_struct=#struct_name{
                     #(
                        #field_name,
                     )*
                  };
                  Ok((data,return_struct))
               }
         }
   
      };
      TokenStream::from(expanded)
   },
   Some(GenericParam::Type(_))=>{
      let expanded = quote! {
         impl<T:Deserialize+Serialize> Deserialize for #struct_name<T>{
            fn deserialize(data:&[u8])-> MapperResult<(&[u8], Self)>{
                  #(
                  let (data,#field_name)=#field_type::deserialize(data)?;
                  )*
                  let return_struct=#struct_name{
                     #(
                        #field_name,
                     )*
                  };
                  Ok((data,return_struct))
               }
         }
   
      };
      TokenStream::from(expanded)
   },
   _=>{
      let expanded = quote! {
         impl Deserialize for #struct_name{
            fn deserialize(data:&[u8])-> MapperResult<(&[u8], Self)>{
                  #(
                  let (data,#field_name)=#field_type::deserialize(data)?;
                  )*
                  let return_struct=#struct_name{
                     #(
                        #field_name,
                     )*
                  };
                  Ok((data,return_struct))
               }
         }
   
      };
      TokenStream::from(expanded)
   },
   }


}


///debug print for enumeration. All element should implement debug macro
#[proc_macro_derive(DebugOutputEnum)]
pub fn implement_debug_print_enum(input: TokenStream) -> TokenStream { 
   let ast = syn::parse_macro_input!(input as DeriveInput);
   implementation_of_debug_print(&ast)

}

fn implementation_of_debug_print(ast: &syn::DeriveInput) -> TokenStream {
let variants= match &ast.data {Data::Enum( DataEnum{variants: v,..})=>v,
   _=> panic!("struct need to have named fields") };

let generic=&ast.generics.params.first();
let field_name = variants.iter().map(|field| &field.ident);
let struct_name = &ast.ident;
match generic {
   Some(GenericParam::Type(_))=>{
      let expanded = quote! {
         impl<T> #struct_name<T>{
            pub fn debug_print(&self) {
               match self {
                  #(
                     #struct_name::#field_name(s)=>println!("{:?}", s),
                  )*
                  }
            }
            }
   
         };
         TokenStream::from(expanded)
   
      },
   Some(GenericParam::Lifetime(_))=>{
   let expanded = quote! {
      impl<'a> #struct_name<'a>{
         pub fn debug_print(&self) {
            match self {
               #(
                  #struct_name::#field_name(s)=>println!("{:?}", s),
               )*
               }
         }
         }

      };
      TokenStream::from(expanded)

   },
   _=>{
      let expanded = quote! {
         impl #struct_name {
            pub fn debug_print(&self) {
               match self {
                  #(
                     #struct_name::#field_name(s)=>{println!("{:?}", s)},
                  )*

                  }
            }
         }

      };
      TokenStream::from(expanded)
   },
   }
    
}