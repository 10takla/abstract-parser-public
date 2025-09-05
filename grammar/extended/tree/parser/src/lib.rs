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

#![feature(phantom_variance_markers, macro_metavar_expr_concat)]

extern crate self as abstract_parser;

use ::parser::{macros, *};
use ::parsers;

const _: () = ();

use crate::Rec;
use abstract_parser::{parsers::chars::InputStreamTrait, rules::MinJoinableRule};
use grammar_core::{parser::*, tree::tree};
use grammar_extended_parser::{
    AnyOrParen, IdentWithDefineGenerics, IdentWithGenerics, quantificator_feature::Comment,
};
use parser::{
    macros::derive_bounds,
    rules::{JoinableRule, OptionalRule, Repeat, SequenceRule},
};
#[allow(unused_imports)]
use parsers::chars::{CharParser, InputStreamIter};
use parsers::chars::{
    macros::{choice_rule, sequence_struct},
    reg_expr_token,
};

pub type Grammar<'src> = grammar_core::parser::Grammar<'src, CommentOrItem<'src>>;

tree! {r#"
    CommentOrItem {
        Comment(Comment)
        Item(Item)
    }
    Item {
        Enum(Enum)
        Struct(Struct)
    }
    Enum {
        head: IdentWithDefineGenericsOrIdent
        Space
        variants: Vars_
    }
"#}
type Vars_<'src> = Braced<'src, Spaced<'src, Vars<'src>>>;

pub type Vars<'src> = JoinableRule<Repeat, Var<'src>, Space<'src>>;

tree! {r#"
    Var {
        ident: Ident
        Space
        value: Value__
    }
"#}
type Value__<'src> = Parened<'src, Spaced<'src, Value<'src>>>;

tree! {r#"
    Struct {
        head: IdentWithDefineGenericsOrIdent
        Space
        fields: StructType
    }
"#}
#[choice_rule(
    OutputAttrs: #[derive_bounds(
        Debug
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        PartialEq
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        Clone
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
    )]
    ErrorAttrs: #[derive_bounds(
        Debug
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        PartialEq
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
        Clone
            <'src, IS: InputStreamTrait<'src>>
            <'src, IS>
    )]
    OutputGenerics: <'src, IS: InputStreamTrait<'src>>
)]
pub enum StructType<'src> {
    Struct(Braced<'src, Spaced<'src, Fields<'src>>>),
    Tuple(
        Parened<
            'src,
            Spaced<
                'src,
                JoinableRule<
                    Repeat,
                    SequenceRule<(
                        OptionalRule<SequenceRule<(Ignored<'src>, Space<'src>)>>,
                        Value<'src>,
                    )>,
                    Space<'src>,
                >,
            >,
        >,
    ),
}
tree! {r#"
    Fields (
        Fields1
        #[ignore] Fields2
    )
"#}
type Fields1<'src> = JoinableRule<Repeat, Field<'src>, Spaced<'src, Comma<'src>>>;
type Fields2<'src> = OptionalRule<SequenceRule<(Space<'src>, Comma<'src>)>>;

tree! {r#"
    Field {
        Named(StructField)
        Unnamed(Value)
    }
    StructField {
        ident: Ident
        SpacedColon
        value: Value
    }
"#}
type SpacedColon<'src> = Spaced<'src, Colon<'src>>;

pub type Value<'src> = Expr<'src>;

tree! {r#"
    Expr {
        Combinator(Combinator)
        Quantificator(Quantificator)
        Token(Token)
    }
    Combinator {
        Choice(Choice)
        Seq(Seq)
    }
    QuantificatorOrToken {
        Quantificator(Quantificator)
        Token(Token)
    }
"#}
pub type Seq<'src> = MinJoinableRule<2, ChoiceOrQuantificator<'src>, Space<'src>>;

pub type ChoiceOrQuantificator<'src> = AnyOrParen<'src, Choice<'src>, QuantificatorOrToken<'src>>;

pub type Choice<'src> =
    MinJoinableRule<2, Rec<QuantificatorOrTokenOrSeq<'src>>, Spaced<'src, Slash<'src>>>;
tree! {r#"
    QuantificatorOrTokenOrSeq {
        QuantificatorOrToken(QuantificatorOrToken)
        Parensized(Parensized)
    }
"#}
pub type Parensized<'src> = Parened<'src, Spaced<'src, Seq<'src>>>;

tree! {r#"
    Quantificator {
        Kleene(Kleene)
        Predicative(Predicative)
        RepeatQuantificator(RepeatQuantificatorExpr)
    }
        Kleene {
            ZeroOrMore(ZeroOrMore)
            OneOrMore(OneOrMore)
        }
            ZeroOrMore ( CombinatorOrToken #[ignore] Space #[ignore] Asterisk )
            OneOrMore ( CombinatorOrToken #[ignore] Space #[ignore] Plus )
        Predicative {
            Optional(Optional)
            NegativeLookahead(NegativeLookahead)
        }
            Optional ( CombinatorOrToken #[ignore] Space #[ignore] QuestionMark  )
            NegativeLookahead ( #[ignore] ExclamationPoint #[ignore] Space CombinatorOrToken )
"#}

#[sequence_struct]
struct RepeatQuantificatorExpr<'src>(
    CombinatorOrToken<'src>,
    #[abstract_parser(ignore)] Space<'src>,
    Braced<'src, Spaced<'src, RepeatQuantificator<'src>>>,
);

pub type CombinatorOrToken<'src> = AnyOrParen<'src, Rec<Combinator<'src>>, Token<'src>>;

tree! {r#"
    RepeatQuantificator {
        Maximum(Maximum)
        MinMax(MinMax)
        Minimum(Minimum)
        Count(Number)
    }
        Minimum (
            Number
            #[ignore] Space
            #[ignore] Comma
        )
        Maximum (
            #[ignore] Space
            #[ignore] Comma
            #[ignore] Space
            Number
        )
        MinMax {
            min: Number
            Space
            Comma
            Space
            max: Number
        }
    Token {
        IdentWithExprGenerics(IdentWithExprGenerics)
        BoxedIdent(BoxedIdent)
        Ident(Ident)
        RegExpr(RegExpr)
    }
"#}
pub type BoxedIdent<'src> = Chevroned<'src, Spaced<'src, Ident<'src>>>;
pub type IdentWithExprGenerics<'src> = IdentWithGenerics<'src, Rec<Expr<'src>>>;

pub type Number<'src> = grammar_core::parser::Number<'src, usize>;

tree! {r#"
    IdentWithDefineGenericsOrIdent {
        IdentWithDefineGenerics(IdentWithDefineGenerics)
        Ident(Ident)
    }
"#}

reg_expr_token! {
    self pub Ignored r"#\[ignore\]"
}
