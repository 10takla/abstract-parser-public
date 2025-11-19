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

use abstract_parser::grammar::core::parser::*;

abstract_parser::grammar::feature::grammar::grammar! {r#"
    Grammar = Command ** NextLine
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
                Fields = FieldName ** Space
                    FieldName {
                        OptItem(OptItem)
                        Base(BaseFieldName)
                    }
                        BaseFieldName {
                            Ident(Ident)
                            Punct(Punct)
                        }
                            Punct {
                                Colon(Colon)
                                Dot(Dot)
                                Comma(Comma)
                            }
                        OptItem {
                            ParenedOpt(Opt<Parened<Spaced<RepeatBaseFieldName>>>)
                            OptIdent(Opt<Ident>)
                        }
                            RepeatBaseFieldName = BaseFieldName **{1,} Space
                FieldDefines = Field_<Tab, Expr> ** NextLine
                    Expr {
                        Choice(Choice)
                        Path(Path)
                        Item(Item)
                    }    
                        Choice {
                            variants: Variants,
                            variant_defines: VariantDefines_?
                        }
                            Variants = Token **{2,} Spaced<Slash>
                            VariantDefines_ (
                                #[ignore] NextLine
                                VariantDefines
                            )
                                VariantDefines = Field_<Tab{2}, RegExpr> **{1,} NextLine
                        Path {
                            command_type: CommandType,
                            command_name: Ident,
                            Dot,
                            field: Ident
                        }
                        Item {
                            Opt(Opt<Token>)
                            Token(Token) 
                        }
    Opt<T> (
        T
        #[ignore] QuestionMark
    )
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
    RegExpr = "\".*\""
    Any = ".*"
    DoubleSlash = "\/\/ "
    Tab = "    "
    Ident = "[A-Za-z_0-9@~]+"
    Space = "\s+"?
    NextLine = "\n"
"#}
