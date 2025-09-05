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

    let output_vars = non_only_rules_vars
        .clone()
        .map(|(_, (ident, ty))| quote!(#ident(<#ty as abstract_parser::TransferRule<IS>>::Output)));

    let error_vars = non_only_rules_vars.clone().map(|(_, (_, ty))| ty);

    let enum_ident = Ident::new(&format!("{ident}Output"), Span::call_site());

    let types = old_vars.clone().into_iter().map(|(_, ty, _)| ty);

    let matches = non_only_rules_vars
        .clone()
        .enumerate()
        .map(|(i, (j, (var_ident, _)))| {
            let arm = {
                let j = Index::from(j);
                quote! {
                    input_stream.parse(&self.#j)
                        .map(#enum_ident ::#var_ident)
                }
            };
            if i == 0 {
                arm
            } else if i == 1 {
                quote!(.or_else(|pre_e| #arm.map_err(|e| (pre_e, e))))
            } else {
                let i = (0..i).map(Literal::usize_unsuffixed);
                quote!(.or_else(|pre_e| #arm.map_err(|e| (#(pre_e.#i),*, e))))
            }
        });

    let choice_error = Ident::new(&format!("{ident}Error"), Span::call_site());

    let choice_error_items = {
        let i = non_only_rules_vars.count();
        if i == 1 {
            quote!(e)
        } else {
            let i = (0..i).map(Index::from);
            quote!(#(e.#i),*)
        }
    };

    let ChoiceAttr {
        output_generics: OutputGenericsField(output_generics),
        tranfer_rule_trait,
        input_stream: InputStreamField(input_stream),
        output_attrs,
        error_attrs,
    } = parse_macro_input!(attr);

    let (_, type_, _) = generics.split_for_impl();

    let mut bounded_generics = bounded_generics(&generics, tranfer_rule_trait);
    bounded_generics.push(parse_quote!(IS: #input_stream));

    let (output_impl, output_type, output_where) = output_generics
        .as_ref()
        .map(|v| {
            let (impl_, type_, where_) = v.split_for_impl();
            (Some(impl_), Some(type_), Some(where_))
        })
        .unwrap_or((None, None, None));

    quote! {
        #[derive(Default)]
        #vis struct #ident #generics (#(pub #types),*);

        #output_attrs
        #vis enum #enum_ident #output_impl #output_where {
            #(#output_vars),*
        }

        #error_attrs
        #vis struct #choice_error #output_impl #output_where (#(pub abstract_parser::ProductionError<<#error_vars as abstract_parser::TransferRule<IS>>::Error>),*);

        impl<#(#bounded_generics),*> abstract_parser::TransferRule<IS> for #ident #type_
        {
            type Output = #enum_ident #output_type #output_where;
            type Error = #choice_error #output_type;

            fn transfer(
                &self,
                input_stream: abstract_parser::InputStream<IS>,
            ) -> Result<Self::Output, abstract_parser::ProductionError<Self::Error>> {
                #(#matches)*
                    .map_err(|e| abstract_parser::ProductionError::Token(#choice_error(#choice_error_items)))
            }
        }
    }
    .into()
}

struct ChoiceAttr {
    tranfer_rule_trait: TranferRuleTraitField,
    input_stream: InputStreamField,
    output_attrs: Option<VecAttrs>,
    error_attrs: Option<VecAttrs>,
    output_generics: OutputGenericsField,
}

impl Parse for ChoiceAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use shared_macros::parse_structs::Field;
        Ok(Self {
            tranfer_rule_trait: Parse::parse(input)?,
            input_stream: Parse::parse(input)?,
            output_attrs: Field::parse(input, "OutputAttrs").ok(),
            error_attrs: Field::parse(input, "ErrorAttrs").ok(),
            output_generics: Parse::parse(input)?,
        })
    }
}
