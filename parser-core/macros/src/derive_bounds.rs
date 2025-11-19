// 
// abstract-parser — proprietary, source-available software (not open-source).    
// Copyright (c) 2025 Abakar Letifov
// (Летифов Абакар Замединович). All rights reserved.
// 
// Use of this Work is permitted only for viewing and internal evaluation,        
// under the terms of the LICENSE file in the repository root.
// If you do not or cannot agree to those terms, do not use this Work.
// 
// THE WORK IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
// 

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Ident, Index, LitStr, Variant, parse::Parse, parse_macro_input,
};

pub fn derive_bounds(attr: TokenStream, input: TokenStream) -> TokenStream {
    let derive_intput = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, data, attrs, ..
    } = &derive_intput;

    let impl_ = {
        parse_macro_input!(attr as Tmps).0.into_iter().map(
            |Tmp {
                 macros,
                 impl_,
                 type_,
             }| {
                if let Data::Union(DataUnion {union_token, ..}) = data { return syn::Error::new_spanned(union_token, "expect enum or struct").to_compile_error() };

                match macros.to_string().as_str() {
                    "Debug" => {
                        let body = match data {
                            Data::Struct(DataStruct { fields, .. }) => {
                                let ident_str = LitStr::new(&ident.to_string(), Span::call_site());
                                match fields {
                                    Fields::Named(FieldsNamed {  named, .. }) => {
                                        let fields = named.into_iter().map(|Field { ident, ..}| {
                                            let field_str = LitStr::new(&ident.clone().unwrap().to_string(), Span::call_site());
                                            quote!(.field(#field_str, &self.#ident))
                                        });
                                        quote!(f.debug_struct(#ident_str) #(#fields)* .finish())
                                    },
                                    Fields::Unnamed(FieldsUnnamed{unnamed,..}, ..) => {
                                        let i = (0..unnamed.len()).map(Index::from);
                                        quote!(f.debug_tuple(#ident_str) #(.field(&self.#i))* .finish())
                                    },
                                    Fields::Unit => todo!(),
                                }
                            },
                            Data::Enum(DataEnum{ variants, .. }) => {
                                let vars = variants.into_iter().map(|Variant { ident: var_ident, fields, .. }| {
                                    match fields {
                                        Fields::Named(..) => todo!(),
                                        Fields::Unnamed(FieldsUnnamed {  unnamed, .. }) => {
                                            let args = unnamed.into_iter().enumerate().map(|(i, _)| Ident::new(&format!("arg{i}"), Span::call_site()));
                                            let args2 = args.clone();
                                            let var_str = LitStr::new(&format!("{ident}::{var_ident}"), Span::call_site());
                                            quote!(Self::#var_ident(#(#args),*) => f.debug_tuple(#var_str) #(.field(#args2))* .finish())
                                        },
                                        Fields::Unit => todo!(),
                                    }
                                });
                                quote!(match self { #(#vars),* })
                            },
                            Data::Union(..) => unreachable!(),
                        };
                        quote! {
                            impl #impl_ std::fmt::Debug for #ident #type_ {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    #body
                                }
                            }
                        }
                    },
                    "PartialEq" => {
                        let body = match data {
                            Data::Struct(DataStruct { fields, .. }) => {
                                match fields {
                                    Fields::Named(FieldsNamed {  named, .. }) => {
                                        let fields = named.into_iter().map(|field| field.ident.as_ref().unwrap());
                                        quote!(#(self.#fields == other.#fields)&&*)
                                    },
                                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                                        let i = (0..unnamed.len()).map(Index::from);
                                        quote!(#(self.#i == other.#i)&&*)
                                    },
                                    Fields::Unit => todo!(),
                                }
                            },
                            Data::Enum(DataEnum{ variants, .. }) => {
                                let vars = variants.into_iter().map(|Variant { ident, fields, .. }| {
                                    match fields {
                                        Fields::Named(..) => todo!(),
                                        Fields::Unnamed(FieldsUnnamed {  unnamed, .. }) => {
                                            let args = unnamed.into_iter().enumerate().map(|(i, _)| (Ident::new(&format!("l{i}"), Span::call_site()), Ident::new(&format!("r{i}"), Span::call_site())));
                                            let (l_args, r_args) = args.clone().unzip::<_, _, Vec<_>, Vec<_>>();
                                            let eqs = args.map(|(l_arg, r_arg)| quote!(#l_arg == #r_arg));
                                            quote!((Self::#ident(#(#l_args),*), Self::#ident(#(#r_args),*)) => #(#eqs)&&*)
                                        },
                                        Fields::Unit => todo!(),
                                    }
                                });
                                quote! {
                                    match (self, other) {
                                        #(#vars),*,
                                        _ => false,
                                    }
                                }
                            },
                            Data::Union(..) => unreachable!()
                        };
                        quote! {
                            impl #impl_ PartialEq for #ident #type_ {
                                fn eq(&self, other: &Self) -> bool {
                                    #body
                                }
                            }
                        }
                    },
                    "Clone" => {
                        let body = match data {
                            Data::Struct(DataStruct { fields, .. }) => {
                                match fields {
                                    Fields::Named(FieldsNamed { named, .. }) => {
                                        let idents = named.into_iter().map(|field| field.ident.as_ref().unwrap());
                                        quote!(Self { #(#idents: self.#idents.clone()),* })
                                    },
                                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                                        let i = (0..unnamed.len()).map(Index::from);
                                        quote!(Self(#(self.#i.clone()),*))
                                    },
                                    Fields::Unit => quote!(Self),
                                }
                            },
                            Data::Enum(DataEnum{ variants, .. }) => {
                                let vars = variants.into_iter().map(|Variant { ident, fields, .. }| {
                                    match fields {
                                        Fields::Named(..) => todo!(),
                                        Fields::Unnamed(FieldsUnnamed {  unnamed, .. }) => {
                                            let args = unnamed.into_iter().enumerate().map(|(i, _)| Ident::new(&format!("arg{i}"), Span::call_site()));
                                            let args2 = args.clone();
                                            quote!(Self::#ident(#(#args),*) => Self::#ident(#(#args2.clone()),*))
                                        },
                                        Fields::Unit => todo!(),
                                    }
                                });
                                quote! {
                                    match self {
                                        #(#vars),*,
                                    }
                                }
                            },
                            Data::Union(..) => unreachable!()
                        };
                        quote! {
                            impl #impl_ Clone for #ident #type_ {
                                fn clone(&self) -> Self {
                                    #body
                                }
                            }
                        }
                    }
                    v => {
                        syn::Error::new_spanned(macros, format!("Not expect {v}"))
                            .to_compile_error()
                    }
                }
            },
        )
    };

    quote! {
        #(#attrs)*
        #derive_intput

        #(#impl_)*
    }
    .into()
}

struct Tmps(Vec<Tmp>);

impl Parse for Tmps {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut vec = vec![];
        while let Ok(v) = Parse::parse(input) {
            vec.push(v);
        }
        Ok(Self(vec))
    }
}

struct Tmp {
    macros: Ident,
    impl_: Generics,
    type_: Generics,
}

impl Parse for Tmp {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            macros: Parse::parse(input)?,
            impl_: Parse::parse(input)?,
            type_: Parse::parse(input)?,
        })
    }
}
