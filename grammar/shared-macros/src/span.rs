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
use parsers::chars::{iter::CharsIter, rules::Chars};

pub struct SynSpanIS<'src, IS> {
    iter: IS,
    span: SynSpan<'src>,
}

impl<'src, IS: InputStreamTrait<'src>> CharParser<'src> for SynSpanIS<'src, IS> {}

impl<'src, IS: Cursorable> Cursorable for SynSpanIS<'src, IS> {
    #[inline]
    fn cursor(&mut self) -> &mut usize {
        self.iter.cursor()
    }
}

impl<'src, IS: Peekab> Peekab for SynSpanIS<'src, IS> {
    #[inline]
    fn peek_n<Error>(
        &mut self,
        offset: usize,
    ) -> Result<Self::Item, parser::ProductionError<Error>> {
        self.iter.peek_n(offset)
    }
}

impl<'src, IS: Iterator> Iterator for SynSpanIS<'src, IS> {
    type Item = IS::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'src, IS: CharsIterTrait<'src>> CharsIterTrait<'src> for SynSpanIS<'src, IS> {
    #[inline]
    fn as_str(&self) -> &'src str {
        self.iter.as_str()
    }
}

pub struct SpanedRule<'src, Rule>(Chars<'src, Rule>);

impl<
        'src,
        IS: InputStreamTrait<'src>,
        Rule: TransferRule<'src, SynSpanIS<'src, IS>, Output = &'src str>,
    > parser::TransferRule<SynSpanIS<'src, IS>> for SpanedRule<'src, Rule>
{
    type Output = (Rule::Output, Span);
    type Error = Rule::Error;

    #[inline]
    fn transfer(
        &self,
        input_stream: parser::InputStream<SynSpanIS<'src, IS>>,
    ) -> Result<Self::Output, parser::ProductionError<Self::Error>> {
        self.0
            .transfer(input_stream)
            .map(|v| (v, input_stream.span.span(v)))
    }
}

pub struct SynSpan<'src> {
    pub src: &'src str,
    pub span: SynSpanV,
}

pub enum SynSpanV {
    Span(Literal),
    CallSite,
}

impl<'src> SynSpan<'src> {
    #[inline]
    pub fn from_str_lit(src: &'src str, str_lit: LitStr) -> Self {
        Self {
            src,
            span: SynSpanV::Span(str_lit.to_token_stream().to_string().parse().unwrap()),
        }
    }

    #[inline]
    pub fn span_result<
        Rule: TransferRule<'src, impl InputStreamTrait<'src>, Output: Debug, Error: Debug>,
    >(
        &self,
        result: Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>>,
    ) -> syn::Result<Rule::Output> {
        result.map_err(|e| syn::Error::new(self.span(e.residue), e))
    }

    pub fn span(&self, sub_str: &'src str) -> Span {
        match &self.span {
            SynSpanV::Span(literal) => {
                let start = sub_str.as_ptr() as usize - self.src.as_ptr() as usize
                    + literal.to_string().find('"').unwrap()
                    + 1;
                literal
                    .subspan(start..start + sub_str.len())
                    .unwrap()
                    .into()
            }
            SynSpanV::CallSite => Span::call_site(),
        }
    }

    #[inline]
    pub fn raw_str_literal(&self, s: &'src str) -> LitStr {
        self.raw_str_literal_with_sub_str(s, s)
    }

    #[inline]
    pub fn raw_str_literal_with_sub_str(&self, s: &str, sub_str: &'src str) -> LitStr {
        self.raw_str_literal_with_span(s, self.span(sub_str))
    }

    #[inline]
    pub fn raw_str_literal_with_span(&self, s: &str, span: Span) -> LitStr {
        let mut v = parse_str::<LitStr>(&raw_str_literal_(s)).unwrap();
        v.set_span(span);
        v
    }
}

#[inline]
pub fn syn_span<
    'src,
    Rule: TransferRule<'src, SynSpanIS<'src, CachedIter<CharsIter<'src>>>, Output: Debug, Error: Debug>,
>(
    str_lit: LitStr,
    src: &'src str,
    rule: &Rule,
) -> syn::Result<Rule::Output> {
    syn_span_result::<Rule>(
        str_lit.clone(),
        src,
        SynSpanIS {
            iter: CachedIter::new(InputStreamIter::new(src)),
            span: SynSpan::from_str_lit(src, str_lit),
        }
        .full_parse(rule),
    )
}

pub fn syn_span_result<
    'src,
    Rule: TransferRule<'src, impl InputStreamTrait<'src>, Output: Debug, Error: Debug>,
>(
    str_lit: LitStr,
    src: &'src str,
    result: Result<Rule::Output, ParseError<'src, Rule::Output, Rule::Error>>,
) -> syn::Result<Rule::Output> {
    result.map_err(|e| {
        syn::Error::new(
            {
                let str_lit = str_lit
                    .to_token_stream()
                    .to_string()
                    .parse::<Literal>()
                    .unwrap();

                let start = e.residue.as_ptr() as usize - src.as_ptr() as usize
                    + str_lit.to_string().find('"').unwrap()
                    + 1;
                str_lit
                    .subspan(start..start + e.residue.len())
                    .unwrap()
                    .into()
            },
            e,
        )
    })
}
