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
        ..
    } = parse_macro_input!(input);

    let (_, type_, where_) = generics.split_for_impl();

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
    } = {
        let attr = attr.clone();
        parse_macro_input!(attr)
    };

    let i = non_only_rule_fields.clone().map(|(i, _)| Index::from(i));

    let (ident, objects, body, output) = match &fields {
        Fields::Named(..) => {
            let output_fields = non_ignored_fields.clone()
                .map(|(_, Field { vis, ident, ty, .. })| {
                    quote!(#vis #ident: <#ty as abstract_parser::TransferRule<IS>>::Output)
                });

            let assembly = non_ignored_fields.map(|(i, Field { ident, .. })| {
                let i = Index::from(i);
                quote!(#ident: v.#i)
            });

            let seq_output = Ident::new(&format!("{ident}Output"), Span::call_site());

            let (output_impl, output_type, output_where) = output_generics
                .as_ref()
                .map(|v| {
                    let (impl_, type_, where_) = v.split_for_impl();
                    (Some(impl_), Some(type_), Some(where_))
                })
                .unwrap_or((None, None, None));

            (
                ident.clone(),
                quote! {
                    #[derive(Default)]
                    pub struct #ident #generics (#(#rule_field_types),*);

                    #(#attrs)*
                    pub struct #seq_output #output_impl {
                        #(#output_fields),*
                    }
                },
                quote! {
                    input_stream.parse(
                        &abstract_parser::rules::SequenceRule((
                            #(&self.#i),*
                        ))
                    )
                    .map(|abstract_parser::rules::SeqOutput(v)| {
                        Self::Output { #(#assembly),* }
                    })
                },
                quote! (#seq_output #output_type #output_where),
            )
        }
        Fields::Unnamed(..) => {
            let output_fields = non_ignored_fields.clone()
                .map(|(_, Field { vis, ty, .. })| {
                    quote!(#vis <#ty as abstract_parser::TransferRule<IS>>::Output)
                });

            let assembly = non_ignored_fields.map(|(i, _)| {
                let i = Index::from(i);
                quote!(v.#i)
            });

            (
                ident.clone(),
                quote! {
                    #[derive(Default)]
                    pub struct #ident #generics (#(#rule_field_types),*);
                },
                quote! {
                    input_stream.parse(&abstract_parser::rules::SequenceRule((
                        #(&self.#i),*
                    )))
                    .map(|abstract_parser::rules::SeqOutput(v)| (#(#assembly),*))
                },
                quote!((#(#output_fields),*)),
            )
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
        quote!(abstract_parser::rules::#seq_error<#(<#error_field_types as abstract_parser::TransferRule<IS>>::Error),*>)
    };

    let mut bounded_generics = bounded_generics(&generics, tranfer_rule_trait);
    bounded_generics.push(parse_quote!(IS: #input_stream));

    quote! {
        #objects

        impl<#(#bounded_generics),*> abstract_parser::TransferRule<IS> for #ident #type_ #where_ {
            type Output = #output;
            type Error = #error;

            fn transfer(
                &self,
                input_stream: abstract_parser::InputStream<IS>,
            ) -> Result<Self::Output, abstract_parser::ProductionError<Self::Error>> {
                #body
            }
        }
    }
    .into()
}

struct SeqAttr {
    tranfer_rule_trait: TranferRuleTraitField,
    input_stream: InputStreamField,
    output_generics: OutputGenericsField,
}

impl Parse for SeqAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tranfer_rule_trait: Parse::parse(input)?,
            input_stream: Parse::parse(input)?,
            output_generics: Parse::parse(input)?,
        })
    }
}
