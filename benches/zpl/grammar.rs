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

use abstract_parser::{
    grammar::{
        core::parser::{Colon, Comma, Dot, Parened, QuestionMark, Slash, Spaced},
        extended::{macros::grammar, tree::macros::tree},
    },
    rules::{JoinableRule, MinJoinableRule, Repeat, SequenceRule},
};

tree! {r#"
    Grammar {
        commands: Commands,
        common_exprs: CommonExpr*
    }
"#}

pub use commands::*;
mod commands {
    use super::*;

    pub type Commands<'src> = JoinableRule<Repeat, Command<'src>, NextLine<'src>>;

    tree! {r#"
        Command {
            comment: Comment,
            command: TypeAndCommand
        }
            TypeAndCommand {
                type_: CommandType,
                command: CommandDef / Ident
            }
            CommandType {
                Control("\^")
                System("~")
            }
            CommandDef {
                name: Define,
                fields: Fields,
                NextLine,
                field_defines: FieldDefines
            }
    "#}

    pub type Fields<'src> = JoinableRule<Repeat, FieldName<'src>, Space<'src>>;
    pub type FieldDefines<'src> = JoinableRule<Repeat, Field<'src>, NextLine<'src>>;

    tree! {r#"
        FieldName {
            OptItem(OptItem)
            Ident(Ident)
            Punct(Punct)
        }
        OptItem {
            OptPunctIdent(Opt<OptPunctIdent_>)
            OptIdent(Opt<Ident>)
        }
        Punct {
            Colon(Colon)
            Dot(Dot)
            Comma(Comma)
        }
        OptPunctIdent (
            Punct
            #[ignore] Space
            Ident
        )
    "#}
    type OptPunctIdent_<'src> = Parened<'src, Spaced<'src, OptPunctIdent<'src>>>;

    grammar! {r#"
        Field = Field_<Tab, Expr>
    "#}

    tree! {r#"
        Expr {
            Choice(Choice)
            Path(Path)
            Item(Item)
        }
    "#}

    tree! {r#"
        Choice {
            variants: Variants,
            NextLine,
            variant_defines: VariantDefines
        }
    "#}

    pub type Variants<'src> = MinJoinableRule<1, Ident<'src>, Spaced<'src, Slash<'src>>>;
    pub type VariantDefines<'src> = MinJoinableRule<1, VariantDefine<'src>, NextLine<'src>>;

    grammar! {r#"
        VariantDefine = Field_<Tab{2}, RegExpr>
            RegExpr = "\".*\""   
    "#}

    tree! {r#"
        Path {
            command: CommandType Ident,
            Dot,
            field: Ident
        }
    "#}
}

tree! {r#"
    CommonExpr {
        NextLine,
        name: Define,
        item: Item
    }
    Item {
        Opt(Opt<Token>)
        Token(Token) 
    }        
    Opt<T> (
        T
        #[ignore] QuestionMark
    )
"#}

tree! {r#"
    Token {
        RegExpr(RegExpr)
        Ident(Ident)
    }
    Field_<Tab, Expr> {
        Tab,
        comment: Comment,
        Tab,
        name: Define,
        expr: Expr
    }
    Comment (
        #[ignore] DoubleSlash
        Any
        #[ignore] NextLine
    )
    Define (
        Ident
        #[ignore] Spaced<"=">
    )
"#}

grammar! {r#"
    Any = ".*"
    DoubleSlash = "\/\/ "
    Tab = "    "
    Ident = "[A-Za-z_0-9@~]+"
    Space = "\s+"?
    NextLine = "\n"
"#}
