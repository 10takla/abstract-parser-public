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

use grammar_core::parser::{Braced, Ident, Parened, Space, Spaced};
use grammar_extended::{macros::grammar, tree::macros::tree};
use parser::rules::{JoinableRule, Repeat};

#[test]
fn tmp() {
    use parsers::chars::{CharParser, InputStreamIter};

    dbg!(InputStreamIter::new(
        r#"
        abc
        #[feature()]
        abc
        "#
    )
    .full_parse(&Grammar::default())
    .unwrap());
}

grammar! {r#"
    Grammar = Spaced<FeatureV*>
"#}

tree! {r#"
    FeatureV {
        Feature(FeatureDef)
        Other((!FeatureHead "[\s\S]")+)
    }
        FeatureDef (
            #[ignore] FeatureHead
            #[ignore] Space
            Parened<Spaced<Features>>
            #[ignore] Space
            #[ignore] "]"s
        )
"#}
pub use feature_head::FeatureHead;
mod feature_head {
    use super::{grammar, Space};
    grammar! {r##"
        FeatureHead = "#"s Space "["s Space "feature"s
    "##}
}
type Features<'src> = JoinableRule<Repeat, Feature<'src>, Space<'src>>;
tree! {r#"
    Feature {
        name: Ident,
        params: (Space Braced<Spaced<Idents>>)?
    }
"#}
type Idents<'src> = JoinableRule<Repeat, Ident<'src>, Space<'src>>;
