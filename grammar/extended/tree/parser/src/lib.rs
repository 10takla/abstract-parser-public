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
    AnyOrParen, IdentWithDefineGenerics, IdentWithGenerics,
    quantificator_feature::{Comment, JoinableRepeat},
};
use parser::{
    macros::derive_bounds,
    rules::{JoinableRule, OptionalRule, Repeat, SequenceRule},
};
use parsers::chars::{
    self,
    macros::{choice_rule, sequence_struct},
    token,
};
#[allow(unused_imports)]
use parsers::chars::{CharParser, InputStreamIter};

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

pub type Vars<'src> = MinJoinableRule<2, Commented<'src, Var<'src>>, Space<'src>>;

tree! {r#"
    Var {
        ident: Ident
        Space
        value: VarValue
    }
"#}
type VarValue<'src> = Parened<'src, Spaced<'src, Expr<'src>>>;

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
    OutputGenerics: <'src, __IS: InputStreamTrait<'src>>
)]
pub enum StructType<'src> {
    Struct(Braced<'src, Spaced<'src, Fields<'src>>>),
    Tuple(Parened<'src, Spaced<'src, MinJoinableRule<2, TupleItem<'src>, Space<'src>>>>),
}

tree! {r#"
    TupleItem {
        Ignored(IgnoredExpr)
        TupleStructExpr(TupleStructExpr)
    }
        IgnoredExpr (
            #[ignore] Ignored
            #[ignore] Space
            IgnoredExprV
        )
            IgnoredExprV {
                TupleStructExpr(TupleStructExpr)
                ParenedSeq(ParenedSeq)
            }
        TupleStructExpr {
            Choice(Choice)
            Quantificator(Quantificator)
            Token(Token)
        }
    Fields (
        Fields1
        #[ignore] FieldEnd
    )
"#}
type Fields1<'src> = MinJoinableRule<2, Commented<'src, Field<'src>>, Spaced<'src, Comma<'src>>>;
type FieldEnd<'src> = OptionalRule<SequenceRule<(Space<'src>, Comma<'src>)>>;

tree! {r#"
    Field {
        Named(NamedField)
        Unnamed(Expr)
    }
    NamedField {
        name: Ident
        SpacedColon
        value: Expr
    }
"#}
type SpacedColon<'src> = Spaced<'src, Colon<'src>>;

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
"#}
pub type Seq<'src> = MinJoinableRule<2, ChoiceOrQuantificator<'src>, Space<'src>>;

pub type ChoiceOrQuantificator<'src> = AnyOrParen<'src, QuantificatorOrToken<'src>, Choice<'src>>;

pub type Choice<'src> =
    MinJoinableRule<2, Rec<QuantificatorOrTokenOrSeq<'src>>, Spaced<'src, Slash<'src>>>;
tree! {r#"
    QuantificatorOrTokenOrSeq {
        QuantificatorOrToken(QuantificatorOrToken)
        ParenedSeq(ParenedSeq)
    }
    QuantificatorOrToken {
        Quantificator(Quantificator)
        Token(Token)
    }
"#}
pub type ParenedSeq<'src> = Parened<'src, Spaced<'src, Seq<'src>>>;

tree! {r#"
    Quantificator {
        Joinable(JoinableExpr)
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

type JoinableExpr<'src> = SequenceRule<(
    CombinatorOrToken<'src>,
    Spaced<'src, Joinable<'src>>,
    CombinatorOrToken<'src>,
)>;
tree! {r#"
    Joinable {
        StrictRepeat(StrictRepeat)
        Repeat(JoinableRepeat)
    }
"#}
#[sequence_struct]
struct StrictRepeat<'src>(
    #[abstract_parser(ignore)] SequenceRule<(JoinableRepeat<'src>, Space<'src>)>,
    Braced<'src, Spaced<'src, RepeatQuantificator<'src>>>,
);

#[sequence_struct]
struct RepeatQuantificatorExpr<'src>(
    CombinatorOrToken<'src>,
    #[abstract_parser(ignore)] Space<'src>,
    Braced<'src, Spaced<'src, RepeatQuantificator<'src>>>,
);

pub type CombinatorOrToken<'src> = AnyOrParen<'src, Token<'src>, Rec<Combinator<'src>>>;

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
        StrLiteral(StrLiteral)
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

token! {
    sub_str self pub Ignored "#[ignore]"
}

#[sequence_struct(
    OutputGenerics: <'src, __IS: InputStreamTrait<'src>, T: chars::TransferRule<'src, __IS>>
)]
#[derive_bounds(
    Debug
        <'src, IS: InputStreamTrait<'src>, T: chars::TransferRule<'src, IS, Output: std::fmt::Debug, Error: std::fmt::Debug>>
        <'src, IS, T>
    Clone
        <'src, IS: InputStreamTrait<'src>, T: chars::TransferRule<'src, IS, Output: Clone, Error: Clone>>
        <'src, IS, T>
    PartialEq
        <'src, IS: InputStreamTrait<'src>, T: chars::TransferRule<'src, IS, Output: PartialEq, Error: PartialEq>>
        <'src, IS, T>
)]
pub struct Commented<'src, T> {
    pub comments: JoinableRule<Repeat, Comment<'src>, Space<'src>>,
    #[abstract_parser(ignore)]
    _i: Space<'src>,
    pub item: T,
}
