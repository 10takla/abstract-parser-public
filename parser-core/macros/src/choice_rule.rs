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
use parser_core::{
    Promotable,
    rules::{OptionalRule, SeqOutput, SequenceRule},
};
use syn::LitStr;
use syn_parser::{InputStreamIter, field_token, rules::SynToken};

pub fn choice_rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemEnum {
        ident,
        variants,
        vis,
        generics,
        ..
    } = parse_macro_input!(input);

    let old_vars = {
        let old_vars = variants
            .iter()
            .map(
                |Variant {
                     ident,
                     fields,
                     attrs,
                     ..
                 }| {
                    let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = fields else {
                        return Err(syn::Error::new_spanned(
                            fields,
                            "Expected unnamed fields (tuple struct variant).",
                        ));
                    };

                    let Field { ty, .. } =
                        unnamed.into_iter().next().ok_or(syn::Error::new_spanned(
                            fields,
                            format!("Variant '{ident}' must have at least one unnamed field."),
                        ))?;

                    Ok((ident, ty, attrs))
                },
            )
            .collect::<Result<Vec<_>, _>>();

        match old_vars {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into(),
        }
    };

    let non_only_rules_vars =
        old_vars
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, (ident, ty, attrs))| {
                if let Some(Meta::List(MetaList { tokens, .. })) = abstarct_parser_attr(attrs)
                    && let Ok(ident) = syn::parse::<Ident>(tokens.clone().into())
                    && ident == "only_rule"
                {
                    None
                } else {
                    Some((i, (ident, ty)))
                }
            });

    let output_vars = non_only_rules_vars.clone().map(
        |(_, (ident, ty))| quote!(#ident(<#ty as abstract_parser::TransferRule<__IS>>::Output)),
    );

    let error_vars = non_only_rules_vars.clone().map(|(_, (_, ty))| ty);

    let types = old_vars.clone().into_iter().map(|(_, ty, _)| ty);

    fn rec_arm<'a>(
        iter: &mut impl Iterator<Item = (usize, (usize, &'a Ident))>,
        last_i: usize,
    ) -> TokenStream2 {
        if let Some((i, (j, var_ident))) = iter.next() {
            let err_arm = rec_arm(iter, i);
            let error_ident = Ident::new(&format!("e{j}"), Span::call_site());
            let j = Index::from(j);
            quote! {
                match input_stream.parse(&self.#j) {
                    Ok(v) => return Ok(Self::Output::#var_ident(v)),
                    Err(#error_ident) => #err_arm
                }
            }
        } else {
            let errs = (0..=last_i).map(|i| Ident::new(&format!("e{i}"), Span::call_site()));
            quote!(__Error(#(#errs),*))
        }
    }

    let mut iter = non_only_rules_vars
        .clone()
        .enumerate()
        .map(|(i, (j, (var_ident, _)))| (i, (j, var_ident)));

    let matches = rec_arm(&mut iter, Default::default());

    let SeqOutput((
        tranfer_rule_bound,
        input_stream_bound,
        output_attrs,
        error_attrs,
        output_generics,
    )) = {
        use crate::fields::Field;
        field_token!(TransferRuleBound InputStreamBound OutputAttrs ErrorAttrs OutputGenerics);
        InputStreamIter::new(attr.clone())
            .parse(&<SequenceRule<(
                OptionalRule<Field<TransferRuleBound, SynToken<TypeParamBound>>>,
                Field<InputStreamBound, SynToken<TypeParamBound>>,
                OptionalRule<Field<OutputAttrs, SynToken<VecAttrs>>>,
                OptionalRule<Field<ErrorAttrs, SynToken<VecAttrs>>>,
                OptionalRule<Field<OutputGenerics, SynToken<Generics>>>,
            )>>::default())
            .unwrap()
    };

    let (impl_, type_, _) = generics.split_for_impl();

    let mut bounded_generics = bounded_generics(&generics, tranfer_rule_bound);
    bounded_generics.push(parse_quote!(__IS: #input_stream_bound));

    let (output_impl, output_type, output_where) = output_generics
        .as_ref()
        .map(|v| {
            let (impl_, type_, where_) = v.split_for_impl();
            (Some(impl_), Some(type_), Some(where_))
        })
        .unwrap_or((None, None, None));

    let mod_name = Ident::new(&format!("__{ident}"), Span::call_site());
    let error_name = Ident::new(&format!("{ident}Error"), Span::call_site());
    let output_name = Ident::new(&format!("{ident}Output"), Span::call_site());

    let debug_impl = {
        let name = LitStr::new(&format!("choice {ident}"), Span::call_site());
        let field_index = non_only_rules_vars.clone().map(|(i, _)| Index::from(i));

        let mut impl_ = generics.clone();
        impl_.type_params_mut().for_each(|v| {
            *v = parse_quote!(#v: std::fmt::Debug);
        });
        quote! {
            impl #impl_ ::core::fmt::Debug for __Rule #generics {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    f.debug_tuple(#name)
                        #( .field(&self. #field_index) )*
                        .finish()
                }
            }
        }
    };

    quote! {
        #vis use self::#mod_name::{Rule as #ident, Output as #output_name, Error as #error_name};
        #[allow(non_snake_case)]
        #vis mod #mod_name {
            pub use __def::{__Rule as Rule, __Output as Output, __Error as Error};
            mod __def {
                use super::super::*;

                #[derive(std::default::Default)]
                pub struct __Rule #generics (#(pub #types),*);

                #debug_impl

                #output_attrs
                pub enum __Output #output_impl #output_where {
                    #(#output_vars),*
                }

                #error_attrs
                pub struct __Error #output_impl #output_where (#(pub abstract_parser::ProductionError<<#error_vars as abstract_parser::TransferRule<__IS>>::Error>),*);

                impl<#(#bounded_generics),*> abstract_parser::TransferRule<__IS> for __Rule #type_
                {
                    type Output = __Output #output_type #output_where;
                    type Error = __Error #output_type;

                    fn transfer(
                        &self,
                        input_stream: abstract_parser::InputStream<__IS>,
                    ) -> Result<Self::Output, abstract_parser::ProductionError<Self::Error>> {
                        Err(abstract_parser::ProductionError::Token(
                            #matches
                        ))
                    }
                }
            }
        }

        impl #impl_ std::fmt::Display for #ident #type_ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {
                write!(
                    f,
                    "{} {}",
                    abstract_parser::utils::logs::SaveLevel::colored("Choice"),
                    stringify!(#ident)
                )
            }
        }
    }
    .into()
}
