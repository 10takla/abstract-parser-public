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

use super::*;

pub fn sequence_struct(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemStruct {
        ident,
        fields,
        generics,
        attrs,
        vis,
        ..
    } = parse_macro_input!(input);

    let (impl_, type_, where_) = generics.split_for_impl();

    let fields_ = match &fields {
        Fields::Named(f) => &f.named,
        Fields::Unnamed(f) => &f.unnamed,
        _ => {
            return syn::Error::new_spanned(&fields, "expect named or unnamed".to_string())
                .to_compile_error()
                .into();
        }
    };

    let rule_field_types = fields_.iter().map(|v| &v.ty).collect::<Vec<_>>();

    let non_only_rule_fields = fields_.into_iter().enumerate().filter_map(|(i, field)| {
        if let Some(Meta::List(MetaList { tokens, .. })) = abstarct_parser_attr(&field.attrs)
            && let Ok(ident) = syn::parse::<Ident>(tokens.clone().into())
            && ident == "only_rule"
        {
            None
        } else {
            Some((i, field))
        }
    });

    let non_ignored_fields =
        non_only_rule_fields
            .clone()
            .enumerate()
            .filter_map(|(i, (_, field))| {
                if let Some(Meta::List(MetaList { tokens, .. })) =
                    abstarct_parser_attr(&field.attrs)
                    && let Ok(ident) = syn::parse::<Ident>(tokens.clone().into())
                    && ident == "ignore"
                {
                    None
                } else {
                    Some((i, field))
                }
            });

    let SeqAttr {
        output_generics: OutputGenericsField(output_generics),
        tranfer_rule_trait,
        input_stream: InputStreamField(input_stream),
        tranfer_rule_generics,
    } = {
        let attr = attr.clone();
        parse_macro_input!(attr)
    };

    let i = non_only_rule_fields.clone().map(|(i, _)| Index::from(i));

    let objects = match &fields {
        Fields::Named(..) => {
            let output_fields = non_ignored_fields.clone()
                .map(|(_, Field { vis, ident, ty, .. })| {
                    quote!(#vis #ident: <#ty as abstract_parser::TransferRule<__IS>>::Output)
                });

            let (output_impl, _, _) = output_generics
                .as_ref()
                .map(|v| {
                    let (impl_, type_, where_) = v.split_for_impl();
                    (Some(impl_), Some(type_), Some(where_))
                })
                .unwrap_or((None, None, None));

            quote! {
                #[derive(std::default::Default, Debug)]
                pub struct __Rule #generics (#(#rule_field_types),*);

                #(#attrs)*
                pub struct __Output #output_impl {
                    #(#output_fields),*
                }
            }
        }
        Fields::Unnamed(..) => {
            quote! {
                #[derive(std::default::Default, Debug)]
                pub struct __Rule #generics (#(#rule_field_types),*);
            }
        }
        _ => unreachable!(),
    };

    let assembly = match &fields {
        Fields::Named(..) => {
            let assembly = non_ignored_fields.clone().map(|(i, Field { ident, .. })| {
                let i = Index::from(i);
                quote!(#ident: v.#i)
            });
            quote!(Self::Output { #(#assembly),* })
        }
        Fields::Unnamed(..) => {
            let assembly = non_ignored_fields.clone().map(|(i, _)| {
                let i = Index::from(i);
                quote!(v.#i)
            });
            quote!((#(#assembly),*))
        }
        _ => unreachable!(),
    };

    let output = match &fields {
        Fields::Named(..) => {
            let (_, output_type, output_where) = output_generics
                .as_ref()
                .map(|v| {
                    let (impl_, type_, where_) = v.split_for_impl();
                    (Some(impl_), Some(type_), Some(where_))
                })
                .unwrap_or((None, None, None));
            quote!(__Output #output_type #output_where)
        }
        Fields::Unnamed(..) => {
            let output_fields = non_ignored_fields.clone()
                .map(|(_, Field { vis, ty, .. })| {
                    quote!(#vis <#ty as abstract_parser::TransferRule<__IS>>::Output)
                });
            quote!((#(#output_fields),*))
        }
        _ => unreachable!(),
    };

    let error = {
        let error_field_types = non_only_rule_fields
            .map(|(_, field)| &field.ty)
            .collect::<Vec<_>>();
        let seq_error = Ident::new(
            &format!("SeqError{}", error_field_types.len()),
            Span::call_site(),
        );
        quote!(abstract_parser::rules::#seq_error<#(<#error_field_types as abstract_parser::TransferRule<__IS>>::Error),*>)
    };

    let mut bounded_generics = bounded_generics(
        if let Some(tranfer_rule_generics) = &tranfer_rule_generics.0 {
            tranfer_rule_generics
        } else {
            &generics
        },
        tranfer_rule_trait,
    );
    bounded_generics.push(parse_quote!(__IS: #input_stream));

    let mod_name = Ident::new(&format!("__{ident}"), Span::call_site());
    let output_name = Ident::new(&format!("{ident}Output"), Span::call_site());

    let output_export = match &fields {
        Fields::Named(..) => Some(quote!(Output as #output_name,)),
        Fields::Unnamed(..) => None,
        _ => unreachable!(),
    };
    let output_export2 = match &fields {
        Fields::Named(..) => Some(quote!(__Output as Output,)),
        Fields::Unnamed(..) => None,
        _ => unreachable!(),
    };

    quote! {
        #vis use self::#mod_name::{Rule as #ident, #output_export};
        #[allow(non_snake_case)]
        #vis mod #mod_name {
            pub use __def::{__Rule as Rule, #output_export2};

            mod __def {
                use super::super::*;

                #objects

                impl<#(#bounded_generics),*> abstract_parser::TransferRule<__IS> for __Rule #type_ #where_ {
                    type Output = #output;
                    type Error = #error;

                    fn transfer(
                        &self,
                        input_stream: abstract_parser::InputStream<__IS>,
                    ) -> Result<Self::Output, abstract_parser::ProductionError<Self::Error>> {
                        input_stream.parse(
                            &abstract_parser::rules::SequenceRule((
                                #(&self.#i),*
                            ))
                        )
                        .map(|abstract_parser::rules::SeqOutput(v)| #assembly)
                    }
                }

                impl #impl_ std::fmt::Display for __Rule #type_ {  
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(
                            f,
                            "{} {}",
                            abstract_parser::utils::logs::SaveLevel::colored("Sequence"),
                            stringify!(#ident)
                        )
                    }
                }
            }
        }
    }
    .into()
}

struct SeqAttr {
    tranfer_rule_trait: TranferRuleTraitField,
    input_stream: InputStreamField,
    output_generics: OutputGenericsField,
    tranfer_rule_generics: TranferRuleGenericsField,
}

impl Parse for SeqAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tranfer_rule_trait: Parse::parse(input)?,
            input_stream: Parse::parse(input)?,
            output_generics: Parse::parse(input)?,
            tranfer_rule_generics: Parse::parse(input)?,
        })
    }
}
