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

use self::{
    system_CC::CC as systemCC, system_CD::CD as systemCD, system_CT::CT as systemCT,
    system_DB::DB as systemDB, system_DE::DE as systemDE, system_DG::DG as systemDG,
    system_DN::DN as systemDN, system_DS::DS as systemDS, system_DT::DT as systemDT,
    system_DU::DU as systemDU, system_DY::DY as systemDY, system_EG::EG as systemEG,
    system_HB::HB as systemHB, system_HD::HD as systemHD, system_HI::HI as systemHI,
    system_HM::HM as systemHM, system_HQ::HQ as systemHQ, system_HS::HS as systemHS,
    system_HU::HU as systemHU, system_JA::JA as systemJA, system_JB::JB as systemJB,
    system_JC::JC as systemJC, system_JD::JD as systemJD, system_JE::JE as systemJE,
    system_JF::JF as systemJF, system_JG::JG as systemJG, system_JI::JI as systemJI,
    system_JL::JL as systemJL, system_JN::JN as systemJN, system_JO::JO as systemJO,
    system_JP::JP as systemJP, system_JQ::JQ as systemJQ, system_JR::JR as systemJR,
    system_JS::JS as systemJS, system_JX::JX as systemJX, system_KB::KB as systemKB,
    system_PH::PH as systemPH, system_PL::PL as systemPL, _A::A, _A_::A_, _B0::B0, _B1::B1,
    _B2::B2, _B3::B3, _B4::B4, _B5::B5, _B7::B7, _B8::B8, _B9::B9, _BA::BA, _BB::BB, _BC::BC,
    _BD::BD, _BE::BE, _BF::BF, _BI::BI, _BJ::BJ, _BK::BK, _BL::BL, _BM::BM, _BO::BO, _BP::BP,
    _BQ::BQ, _BR::BR, _BS::BS, _BT::BT, _BU::BU, _BX::BX, _BY::BY, _BZ::BZ, _CC::CC, _CD::CD,
    _CF::CF, _CI::CI, _CM::CM, _CN::CN, _CO::CO, _CP::CP, _CT::CT, _CV::CV, _CW::CW, _DF::DF,
    _FB::FB, 
    _FC::FC, _FD::FD, _FE::FE, _FH::FH, _FL::FL, _FM::FM, _FN::FN, _FO::FO, _FP::FP,
    _FR::FR, _FS::FS, _FT::FT, _FV::FV, _FW::FW, _FX::FX, _GB::GB, _GC::GC, _GD::GD, _GE::GE,
    _GF::GF, _GS::GS, _HF::HF, _HG::HG, _HH::HH, _HT::HT, _HV::HV, _HW::HW, _HY::HY, _HZ::HZ,
    _HZO::HZO, _ID::ID, _IL::IL, _IM::IM, _IS::IS, _JB::JB, _JH::JH, _JI::JI, _JJ::JJ, _JM::JM,
    _JS::JS, _JT::JT, _JU::JU, _JW::JW, _JZ::JZ, _KD::KD, _KL::KL, _KN::KN, _KP::KP, _KV::KV,
    _LF::LF, _LH::LH, _LL::LL, _LR::LR, _LS::LS, _LT::LT, _MA::MA, _MC::MC, _MD::MD, _MF::MF,
    _MI::MI, _ML::ML, _MM::MM, _MN::MN, _MP::MP, _MT::MT, _MU::MU, _MW::MW, _PA::PA, _PF::PF,
    _PH::PH, _PM::PM, _QRFD::QRFD,
};
abstract_parser::grammar::core::macros::grammar! {
    r#"AllCommands = A / A_ / B0 / B1 / B2 / B3 / B4 / B5 / B7 / B8 / B9 / BA / BB / BC / BD / BE / BF / BI / BJ / BK / BL / BM / BO / BP / BQ / QRFD / BR / BS / BT / BU / BX / BY / BZ / CC / systemCC / CD / systemCD / CF / CI / CM / CN / CO / CP / CT / systemCT / CV / CW / systemDB / systemDE / DF / systemDG / systemDN / systemDS / systemDT / systemDU / systemDY / systemEG / FB / FC / FD / FE / FH / FL / FM / FN / FO / FP / FR / FS / FT / FV / FW / FX / GB / GC / GD / GE / GF / GS / systemHB / systemHD / HF / HG / HH / systemHI / systemHM / systemHQ / systemHS / HT / systemHU / HV / HW / HY / HZ / HZO / ID / IL / IM / IS / systemJA / JB / systemJB / systemJC / systemJD / systemJE / systemJF / systemJG / JH / JI / systemJI / JJ / systemJL / JM / systemJN / systemJO / systemJP / systemJQ / systemJR / JS / systemJS / JT / JU / JW / systemJX / JZ / systemKB / KD / KL / KN / KP / KV / LF / LH / LL / LR / LS / LT / MA / MC / MD / MF / MI / ML / MM / MN / MP / MT / MU / MW / PA / PF / PH / systemPH / systemPL / PM"#
}
pub mod _A { 
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
A = "\^A" A_fields
    A_fields = f ("," o)? ("," h)? ("," w)?
		f = "[A-Z0-9]"
		o = "[NRIB]"
		h = Scalable / Bitmapped
		Scalable = "(1[0-9]|[2-9][0-9]|[1-9][0-9]{2,3}|[12][0-9]{4}|3[01][0-9]{3}|32000)"
		Bitmapped = "([1-9]|10)"
		w = h
"#
    }
}
pub mod _A_ {
    use super::{
        common_exprs::*,
        _A::{h, o, w},
    };
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
A_ = "\^A@" A__fields
    A__fields = o? ("," h)? ("," w)? "," d ":" f "/." x
		d = REBA
		f = "[A-Za-z0-9_]+"
		x = "FNT" / "TTF" / "TTE"
"#
    }
}
pub mod _B0 {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B0 = "\^B0" B0_fields
    B0_fields = a? ("," b)? ("," c)? ("," d)? ("," e)? ("," f)? ("," g)?
		a = o
		b = "([1-9]|10)"
		c = YesNo
		d = "(0|[0-9]{2}|10[1-4]|2(0[1-9]|[12][0-9]|3[0-2])|300)"
		e = YesNo
		f = "([1-9]|1[0-9]|2[0-6])"
		g = "[ -~]{0,24}"
"#
    }
}
pub mod _B1 {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B1 = "\^B1" B1_fields
    B1_fields = o? ("," e)? ("," h)? ("," f)? ("," g)?
		e = YesNo
		h = "([1-9][0-9]{0,3}|[12][0-9]{4}|3[01][0-9]{3}|32000)"
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _B2 {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B2 = "\^B2" B2_fields
    B2_fields = o? ("," h)? ("," f)? ("," g)? ("," e)? ("," j)?
		f = YesNo
		g = YesNo
		e = YesNo
		j = e
"#
    }
}
pub mod _B3 {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B3 = "\^B3" B3_fields
    B3_fields = o? ("," e)? ("," h)? ("," f)? ("," g)?
		e = YesNo
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _B4 {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B4 = "\^B4" B4_fields
    B4_fields = o? ("," h)? ("," f)? ("," m)?
		h = "([1-9][0-9]{0,3}|[12][0-9]{4}|3[01][0-9]{3}|32000)"
		f = "[NAB]"
		m = "[0-5A]"
"#
    }
}
pub mod _B5 {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B5 = "\^B5" B5_fields
    B5_fields = o? ("," h)? ("," f)? ("," g)?
		h = "([1-9][0-9]{0,3}|9999)"
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _B7 {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B7 = "\^B7" B7_fields
    B7_fields = o? ("," h)? ("," s)? ("," c)? ("," r)? ("," t)?
		h = "([1-9][0-9]{0,3}|[12][0-9]{4}|30000)"
		s = "[1-8]"
		c = "([1-9]|[12][0-9]|30)"
		r = "([3-9]|[1-8][0-9]|90)"
		t = YesNo
"#
    }
}
pub mod _B8 {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B8 = "\^B8" B8_fields
    B8_fields = o? ("," h)? ("," f)? ("," g)?
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _B9 {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
B9 = "\^B9" B9_fields
    B9_fields = o? ("," h)? ("," f)? ("," g)? ("," e)?
		f = YesNo
		g = YesNo
		e = YesNo
"#
    }
}
pub mod _BA {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BA = "\^BA" BA_fields
    BA_fields = o? ("," h)? ("," f)? ("," g)? ("," e)?
		f = YesNo
		g = YesNo
		e = YesNo
"#
    }
}
pub mod _BB {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BB = "\^BB" BB_fields
    BB_fields = o? ("," h)? ("," s)? ("," c)? ("," r)? ("," m)?
		h = "([2-9]|[1-9][0-9]{1,3}|[12][0-9]{4}|32000)"
		s = YesNo
		c = "([2-9]|[1-5][0-9]|6[0-2])"
		r = "([1-9]|1[0-9]|2[0-2])" / "[2-4]"
		m = "[AEF]"
"#
    }
}
pub mod _BC {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BC = "\^BC" BC_fields
    BC_fields = o? ("," h)? ("," f)? ("," g)? ("," e)? ("," m)?
		f = YesNo
		g = YesNo
		e = YesNo
		m = "[NUAD]"
"#
    }
}
pub mod _BD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BD = "\^BD" BD_fields
    BD_fields = m? ("," n)? ("," t)?
		m = "[2-6]"
		n = "[1-8]"
		t = "[1-8]"
"#
    }
}
pub mod _BE {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BE = "\^BE" BE_fields
    BE_fields = o? ("," h)? ("," f)? ("," g)?
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _BF {
    use super::{common_exprs::*, _A::o, _B5::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BF = "\^BF" BF_fields
    BF_fields = o? ("," h)? ("," m)?
		m = "([0-9]|[1-9][0-9]|3[0-3])"
"#
    }
}
pub mod _BI {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BI = "\^BI" BI_fields
    BI_fields = o? ("," h)? ("," f)? ("," g)?
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _BJ {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BJ = "\^BJ" BJ_fields
    BJ_fields = o? ("," h)? ("," f)? ("," g)?
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _BK {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BK = "\^BK" BK_fields
    BK_fields = o? ("," e)? ("," h)? ("," f)? ("," g)? ("," k)? ("," l)?
		e = "N"
		f = YesNo
		g = YesNo
		k = "[ABCD]"
		l = k
"#
    }
}
pub mod _BL {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BL = "\^BL" BL_fields
    BL_fields = o? ("," h)? ("," g)?
		g = YesNo
"#
    }
}
pub mod _BM {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BM = "\^BM" BM_fields
    BM_fields = o? ("," e)? ("," h)? ("," f)? ("," g)? ("," e2)?
		e = "[ABCD]"
		f = YesNo
		g = YesNo
		e2 = YesNo
"#
    }
}
pub mod _BO {
    use super::{common_exprs::*, _A::o, _B0::b};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BO = "\^BO" BO_fields
    BO_fields = a? ("," b)? ("," c)? ("," d)? ("," e)? ("," f)? ("," g)?
		a = o
		c = YesNo
		d = "(0|[0-9]{2}|10[1-4]|2(0[1-9]|[12][0-9]|3[0-2])|300)"
		e = YesNo
		f = "([1-9]|1[0-9]|2[0-6])"
		g = "[ -~]{0,24}"
"#
    }
}
pub mod _BP {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BP = "\^BP" BP_fields
    BP_fields = o? ("," e)? ("," h)? ("," f)? ("," g)?
		e = YesNo
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _BQ {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BQ = "\^BQ" BQ_fields
    BQ_fields = a? ("," b)? ("," c)? ("," d)? ("," e)?
		a = "N"
		b = "[12]"
		c = "([1-9][0-9]?|100)"
		d = "[HQML]"
		e = "[0-7]"
"#
    }
}
pub mod _QRFD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"QRFD = "\^QRFD""# }
}
pub mod _BR {
    use super::{common_exprs::*, _A::o, _B0::b, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BR = "\^BR" BR_fields
    BR_fields = a? ("," b)? ("," c)? ("," d)? ("," e)? ("," f)?
		a = o
		b = "(1[0-2]|[1-9])"
		c = b
		d = "[12]"
		e = h
		f = "(2|4|6|8|10|12|14|16|18|20|22)"
"#
    }
}
pub mod _BS {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BS = "\^BS" BS_fields
    BS_fields = o? ("," h)? ("," f)? ("," g)?
		f = YesNo
		g = YesNo
"#
    }
}
pub mod _BT {
    use super::{common_exprs::*, _A::o, _B0::b, _B5::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BT = "\^BT" BT_fields
    BT_fields = o? ("," w1)? ("," r1)? ("," h1)? ("," w2)? ("," h2)?
		w1 = b
		r1 = "(2(\.0)?|2\.[1-9]|3(\.0)?)"
		h1 = h
		w2 = w1
		h2 = "([1-9][0-9]{0,2}|255)"
"#
    }
}
pub mod _BU {
    use super::{common_exprs::*, _A::o, _B5::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BU = "\^BU" BU_fields
    BU_fields = o? ("," h)? ("," f)? ("," g)? ("," e)?
		f = YesNo
		g = YesNo
		e = YesNo
"#
    }
}
pub mod _BX {
    use super::{common_exprs::*, _A::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BX = "\^BX" BX_fields
    BX_fields = o? ("," h)? ("," s)? ("," c)? ("," r)? ("," f)? ("," g)? ("," a)?
		h = "([1-9][0-9]{0,3}|[12][0-9]{4}|32000)"
		s = "(0|50|80|100|140|200)"
		c = "([9-9]|[1-4][0-9])"
		r = "([9-9]|[1-4][0-9])"
		f = "[1-6]"
		g = "."
		a = "[12]"
"#
    }
}
pub mod _BY {
    use super::{common_exprs::*, _B0::b};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BY = "\^BY" BY_fields
    BY_fields = w? ("," r)? ("," h)?
		w = b
		r = "(2(\.0)?|2\.[0-9]|3(\.0)?)"
		h = "([1-9][0-9]{0,3}|[12][0-9]{4}|32000)"
"#
    }
}
pub mod _BZ {
    use super::{common_exprs::*, _A::o, _B1::h};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
BZ = "\^BZ" BZ_fields
    BZ_fields = o? ("," h)? ("," f)? ("," g)? ("," t)?
		f = YesNo
		g = YesNo
		t = "[0123]"
"#
    }
}
pub mod _CC {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CC = "\^CC" CC_fields
    CC_fields = x
		x = "."
"# }
}
pub mod system_CC {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CC = "~CC" CC_fields
    CC_fields = x
		x = "."
"# }
}
pub mod _CD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CD = "\^CD" CD_fields
    CD_fields = a
		a = "."
"# }
}
pub mod system_CD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CD = "~CD" CD_fields
    CD_fields = a
		a = "."
"# }
}
pub mod _CF {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
CF = "\^CF" CF_fields
    CF_fields = f? ("," h)? ("," w)?
		f = "[A-Z0-9]"
		h = "([0-9]|[1-9][0-9]{1,3}|[12][0-9]{4}|32000)"
		w = h
"#
    }
}
pub mod _CI {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"CI = "\^CI""# }
}
pub mod _CM {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
CM = "\^CM" CM_fields
    CM_fields = a? ("," b)? ("," c)? ("," d)? ("," e)?
		a = "[B|E|R|A]|NONE"
		b = a
		c = a
		d = a
		e = "M"
"#
    }
}
pub mod _CN {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CN = "\^CN" CN_fields
    CN_fields = a
		a = "[01]"
"# }
}
pub mod _CO {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
CO = "\^CO" CO_fields
    CO_fields = a? ("," b)? ("," c)?
		a = YesNo
		b = "([1-9][0-9]{0,3}|[1-9][0-9]{4})"
		c = "[01]"
"#
    }
}
pub mod _CP {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CP = "\^CP" CP_fields
    CP_fields = a
		a = "[012]"
"# }
}
pub mod _CT {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CT = "\^CT" CT_fields
    CT_fields = a
		a = "."
"# }
}
pub mod system_CT {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CT = "~CT" CT_fields
    CT_fields = a
		a = "."
"# }
}
pub mod _CV {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
CV = "\^CV" CV_fields
    CV_fields = a
		a = YesNo
"# }
}
pub mod _CW {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"CW = "\^CW""# }
}
pub mod system_DB {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"DB = "~DB""# }
}
pub mod system_DE {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DE = "~DE" DE_fields
    DE_fields = d ":" o "/." x "," s "," data
		d = REBA
		o = "[A-Za-z0-9]{1,8}"
		x = "DAT"
		s = "[1-9][0-9]*"
		data = "([A-F0-9]{4}\s*)+"
"#
    }
}
pub mod _DF {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DF = "\^DF" DF_fields
    DF_fields = d ":" o "/." x
		d = REBA
		o = "[A-Za-z0-9]{1,16}"
		x = "ZPL"
"#
    }
}
pub mod system_DG {
    use super::{
        common_exprs::*,
        system_DE::{o, s},
    };
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DG = "~DG" DG_fields
    DG_fields = d ":" o "/." x "," t "," w "," data
		d = REBA
		x = "GRF"
		t = s
		w = t
		data = "[A-F0-9]+"
"#
    }
}
pub mod system_DN {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"DN = "~DN""# }
}
pub mod system_DS {
    use super::{
        common_exprs::*,
        system_DE::{o, s},
        system_DG::data,
    };
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DS = "~DS" DS_fields
    DS_fields = d ":" o "/." x "," s "," data
		d = REBA
		x = "FNT"
"#
    }
}
pub mod system_DT {
    use super::{
        common_exprs::*,
        system_DE::{o, s},
        system_DG::data,
    };
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DT = "~DT" DT_fields
    DT_fields = d ":" o "," s "," data
		d = REBA
		x = "DAT"
"#
    }
}
pub mod system_DU {
    use super::{
        common_exprs::*,
        system_DE::{o, s},
        system_DG::data,
    };
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DU = "~DU" DU_fields
    DU_fields = d ":" o "/." x "," s "," data
		d = REBA
		x = "FNT"
"#
    }
}
pub mod system_DY {
    use super::{
        common_exprs::*,
        system_DE::{o, s},
    };
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
DY = "~DY" DY_fields
    DY_fields = d ":" f "," b "," x "," t "," w "," data
		d = REBA
		f = o
		b = "[ABCP]"
		x = "[BEGPTX]|NRD|PAC|C|F|H"
		t = s
		w = "[0-9]*"
		data = "([A-F0-9]+|[A-Za-z0-9+/=]+)?"
"#
    }
}
pub mod system_EG {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"EG = "~EG""# }
}
pub mod _FB {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
FB = "\^FB" FB_fields
    FB_fields = a? ("," b)? ("," c)? ("," d)? ("," e)?
		a = "[0-9]+"
		b = "[1-9][0-9]{0,3}"
		c = "-?[0-9]+"
		d = "[LCRJ]"
		e = a
"#
    }
}
pub mod _FC {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
FC = "\^FC" FC_fields
    FC_fields = a ("," b)? ("," c)?
		a = "."
		b = "."
		c = "."
"#
    }
}
pub mod _FD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
FD = "\^FD" FD_fields
    FD_fields = a
		a = ".{1,3072}"
"# }
}
pub mod _FE {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
FE = "\^FE" FE_fields
    FE_fields = a
		a = "[^\\^~]"
"# }
}
pub mod _FH {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
FH = "\^FH" FH_fields
    FH_fields = a?
		a = "[^\\^~]"
"# }
}
pub mod _FL {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"FL = "\^FL""# }
}
pub mod _FM {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"FM = "\^FM""# }
}
pub mod _FN {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"FN = "\^FN""# }
}
pub mod _FO {
    use super::{common_exprs::*, _CP::a};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
FO = "\^FO" FO_fields
    FO_fields = x? ("," y)? ("," z)?
		x = "[0-9]{1,5}"
		y = x
		z = a
"#
    }
}
pub mod _FP {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
FP = "\^FP" FP_fields
    FP_fields = d? ("," g)?
		d = "[HVR]"
		g = "([0-9]{1,4})"
"#
    }
}
pub mod _FR {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"FR = "\^FR""# }
}
pub mod _FS {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"FS = "\^FS""# }
}
pub mod _FT {
    use super::{common_exprs::*, _CP::a, _FO::x};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
FT = "\^FT" FT_fields
    FT_fields = x? ("," y)? ("," z)?
		y = x
		z = a
"#
    }
}
pub mod _FV {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
FV = "\^FV" FV_fields
    FV_fields = a
		a = ".{0,3072}"
"# }
}
pub mod _FW {
    use super::{common_exprs::*, _A::o, _CP::a};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
FW = "\^FW" FW_fields
    FW_fields = r? ("," z)?
		r = o
		z = a
"#
    }
}
pub mod _FX {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
FX = "\^FX" FX_fields
    FX_fields = c
		c = ".*"
"# }
}
pub mod _GB {
    use super::{common_exprs::*, _FO::x};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
GB = "\^GB" GB_fields
    GB_fields = w? ("," h)? ("," t)? ("," c)? ("," r)?
		w = x
		h = w
		t = "([1-9][0-9]{0,4})"
		c = "[BW]"
		r = "[0-8]"
"#
    }
}
pub mod _GC {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
GC = "\^GC" GC_fields
    GC_fields = d? ("," t)? ("," c)?
		d = "([3-9]|[1-9][0-9]{1,3}|[12][0-9]{4}|4095)"
		t = "([1-9][0-9]{0,3}|4095)"
		c = "[BW]"
"#
    }
}
pub mod _GD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
GD = "\^GD" GD_fields
    GD_fields = w? ("," h)? ("," t)? ("," c)? ("," o)?
		w = "([3-9]|[1-9][0-9]{1,3}|[12][0-9]{4}|32000)"
		h = w
		t = "([1-9][0-9]{0,3}|32000)"
		c = "[BW]"
		o = "[R\\/]"
"#
    }
}
pub mod _GE {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
GE = "\^GE" GE_fields
    GE_fields = w? ("," h)? ("," t)? ("," c)?
		w = "([3-9]|[1-9][0-9]{1,2}|[12][0-9]{3}|3[0-9]{3}|4095)"
		h = w
		t = "([1-9][0-9]{0,2}|[12][0-9]{3}|4095)"
		c = "[BW]"
"#
    }
}
pub mod _GF {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
GF = "\^GF" GF_fields
    GF_fields = a "," b "," c "," d "," data
		a = "[ABC]"
		b = "([1-9][0-9]{0,4})"
		c = b
		d = b
		data = "([A-F0-9,\r\n]+)"
"#
    }
}
pub mod _GS {
    use super::{common_exprs::*, _A::o, _FO::x};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
GS = "\^GS" GS_fields
    GS_fields = o? ("," h)? ("," w)?
		h = x
		w = h
"#
    }
}
pub mod system_HB {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HB = "~HB""# }
}
pub mod system_HD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HD = "~HD""# }
}
pub mod _HF {
    use super::{common_exprs::*, system_DE::o, _DF::x};
    abstract_parser::grammar::extended::macros::grammar! { r#"
HF = "\^HF" HF_fields
    HF_fields = d ":" o "/." x
		d = REBA
"# }
}
pub mod _HG {
    use super::{common_exprs::*, system_DE::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
HG = "\^HG" HG_fields
    HG_fields = d ":" o "/." x
		d = REBA
		x = "GRF"
"#
    }
}
pub mod _HH {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HH = "\^HH""# }
}
pub mod system_HI {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HI = "~HI""# }
}
pub mod system_HM {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HM = "~HM""# }
}
pub mod system_HQ {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
HQ = "~HQ" HQ_fields
    HQ_fields = query
		query = "ES" / "HA" / "JT" / "MA" / "MI" / "OD" / "PH" / "PP" / "SN" / "UI"
"#
    }
}
pub mod system_HS {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HS = "~HS""# }
}
pub mod _HT {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HT = "\^HT""# }
}
pub mod system_HU {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"HU = "~HU""# }
}
pub mod _HV {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
HV = "\^HV" HV_fields
    HV_fields = n ("," b)? ("," h)? ("," t)? ("," a)?
		n = "([0-9]{1,4})"
		b = "([1-9][0-9]{0,2}|256)"
		h = ".{0,3072}"
		t = ".{0,3072}"
		a = "[FL]"
"#
    }
}
pub mod _HW {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
HW = "\^HW" HW_fields
    HW_fields = d ":" o "/." x ("," f)?
		d = "[REBAZ]"
		o = "[A-Za-z0-9?*]{1,8}"
		x = "[A-Za-z0-9?*]{1,3}"
		f = "[cd]"
"#
    }
}
pub mod _HY {
    use super::{common_exprs::*, system_DE::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
HY = "\^HY" HY_fields
    HY_fields = d ":" o "/." x
		d = REBA
		x = "GRF" / "PNG"
"#
    }
}
pub mod _HZ {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
HZ = "\^HZ" HZ_fields
    HZ_fields = b
		b = "[aflor]"
"# }
}
pub mod _HZO {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
HZO = "\^HZO" HZO_fields
    HZO_fields = "," d ":" o "/." x ("," l)?
		d = REBA
		o = "[A-Za-z0-9]{1,16}"
		x = "(FNT|GRF|PNG|ZPL|DAT|ZOB|STO)"
		l = "[YN]"
"#
    }
}
pub mod _ID {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
ID = "\^ID" ID_fields
    ID_fields = d ":" o "/." x
		d = REBA
		o = "[A-Za-z0-9*]{1,8}"
		x = "[A-Za-z0-9*]{1,3}"
"#
    }
}
pub mod _IL {
    use super::{common_exprs::*, system_DE::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
IL = "\^IL" IL_fields
    IL_fields = d ":" o "/." x
		d = REBA
		x = "(GRF|PNG)"
"#
    }
}
pub mod _IM {
    use super::{common_exprs::*, system_DE::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
IM = "\^IM" IM_fields
    IM_fields = d ":" o "/." x
		d = REBA
		x = "(GRF|PNG)"
"#
    }
}
pub mod _IS {
    use super::{common_exprs::*, system_DE::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
IS = "\^IS" IS_fields
    IS_fields = d ":" o "/." x ("," p)?
		d = REBA
		x = "(GRF|PNG)"
		p = YesNo
"#
    }
}
pub mod system_JA {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JA = "~JA""# }
}
pub mod _JB {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JB = "\^JB" JB_fields
    JB_fields = a
		a = "[ABE]"
"# }
}
pub mod system_JB {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JB = "~JB""# }
}
pub mod system_JC {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JC = "~JC""# }
}
pub mod system_JD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JD = "~JD""# }
}
pub mod system_JE {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JE = "~JE""# }
}
pub mod system_JF {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JF = "~JF" JF_fields
    JF_fields = p
		p = YesNo
"# }
}
pub mod system_JG {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JG = "~JG""# }
}
pub mod _JH {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
JH = "\^JH" JH_fields
    JH_fields = a? ("," b)? ("," c)? ("," d)? ("," e)? ("," f)? ("," g)? ("," h)? ("," i)? ("," j)?
		a = "[ED]"
		b = "([1-9][0-9]{2,3}|9999)"
		c = YesNo
		d = "(N|[0-9]{1,2})"
		e = YesNo
		f = "[ED]"
		g = "(0|[1-9]|1[0-6])"
		h = YesNo
		i = "([0-9]{2,7})"
		j = YesNo
"#
    }
}
pub mod _JI {
    use super::{common_exprs::*, system_DE::o};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
JI = "\^JI" JI_fields
    JI_fields = d ":" o "/." x ("," b)? ("," c)? ("," m)?
		d = REBA
		x = "(BAS|BAE)"
		b = YesNo
		c = YesNo
		m = "([2-9][0-9]K|[1-9][0-9]{2,3}K|1024K)"
"#
    }
}
pub mod system_JI {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JI = "~JI""# }
}
pub mod _JJ {
    use super::{common_exprs::*, _CP::a};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
JJ = "\^JJ" JJ_fields
    JJ_fields = a? ("," b)? ("," c)? ("," d)? ("," e)? ("," f)?
		b = "[0-4]"
		c = "[pl]"
		d = "[ef]"
		e = "[ed]"
		f = e
"#
    }
}
pub mod system_JL {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JL = "~JL""# }
}
pub mod _JM {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JM = "\^JM" JM_fields
    JM_fields = n
		n = "[AB]"
"# }
}
pub mod system_JN {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JN = "~JN""# }
}
pub mod system_JO {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JO = "~JO""# }
}
pub mod system_JP {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JP = "~JP""# }
}
pub mod system_JQ {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JQ = "~JQ""# }
}
pub mod system_JR {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JR = "~JR""# }
}
pub mod _JS {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JS = "\^JS" JS_fields
    JS_fields = a
		a = "[ART]"
"# }
}
pub mod system_JS {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
JS = "~JS" JS_fields
    JS_fields = b
		b = "A" / "B" / "N" / "O" / "(10|20|30|40|50|60|70|80|90|100)"
"#
    }
}
pub mod _JT {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
JT = "\^JT" JT_fields
    JT_fields = n ("," a)? ("," b)? ("," c)?
		n = "([0-9]{4})"
		a = YesNo
		b = "([0-9]{1,4})"
		c = b
"#
    }
}
pub mod _JU {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JU = "\^JU" JU_fields
    JU_fields = a
		a = "[AFNRS]"
"# }
}
pub mod _JW {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JW = "\^JW" JW_fields
    JW_fields = t
		t = "[LMH]"
"# }
}
pub mod system_JX {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"JX = "~JX""# }
}
pub mod _JZ {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
JZ = "\^JZ" JZ_fields
    JZ_fields = a
		a = YesNo
"# }
}
pub mod system_KB {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"KB = "~KB""# }
}
pub mod _KD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
KD = "\^KD" KD_fields
    KD_fields = a
		a = "[0-4]"
"# }
}
pub mod _KL {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
KL = "\^KL" KL_fields
    KL_fields = a
		a = "(1[0-9]|20|[1-9])"
"#
    }
}
pub mod _KN {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
KN = "\^KN" KN_fields
    KN_fields = a? ("," b)?
		a = "[A-Za-z0-9]{0,16}"
		b = "[A-Za-z0-9]{0,35}"
"#
    }
}
pub mod _KP {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
KP = "\^KP" KP_fields
    KP_fields = a ("," b)?
		a = "[0-9]{4}"
		b = "[1-4]"
"#
    }
}
pub mod _KV {
    use super::{common_exprs::*, _CP::a};
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
KV = "\^KV" KV_fields
    KV_fields = a? ("," b)? ("," c)? ("," d)? ("," e)?
		a = "(0|[1-5][0-9]|60)"
		b = "[2-9]"
		c = a
		d = "([0-9]|[1-9][0-9]{1,2}|[12][0-9]{3}|300)"
		e = "([0-9]|[1-9][0-9]{1,2}|[1-9][0-9]{3}|1023)"
"#
    }
}
pub mod _LF {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"LF = "\^LF""# }
}
pub mod _LH {
    use super::{common_exprs::*, _FO::x};
    abstract_parser::grammar::extended::macros::grammar! { r#"
LH = "\^LH" LH_fields
    LH_fields = x? ("," y)?
		y = x
"# }
}
pub mod _LL {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
LL = "\^LL" LL_fields
    LL_fields = y ("/." x)?
		y = "([1-9][0-9]{0,4}|32000)"
		x = YesNo
"#
    }
}
pub mod _LR {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
LR = "\^LR" LR_fields
    LR_fields = a
		a = YesNo
"# }
}
pub mod _LS {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
LS = "\^LS" LS_fields
    LS_fields = a
		a = "(-?[0-9]{1,4})"
"# }
}
pub mod _LT {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
LT = "\^LT" LT_fields
    LT_fields = x
		x = "(-?[0-9]{1,4})"
"# }
}
pub mod _MA {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MA = "\^MA" MA_fields
    MA_fields = type_ "," print "," threshold "," frequency "," units
		type_ = "[RC]"
		print = YesNo
		threshold = "([0-9]{1,4})"
		frequency = "([0-9]{1,4})"
		units = "[CIM]"
"#
    }
}
pub mod _MC {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
MC = "\^MC" MC_fields
    MC_fields = a
		a = YesNo
"# }
}
pub mod _MD {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MD = "\^MD" MD_fields
    MD_fields = a
		a = "(-?[0-9]{1,2}(\.[0-9])?)"
"#
    }
}
pub mod _MF {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MF = "\^MF" MF_fields
    MF_fields = p? ("," h)?
		p = "[FCLNS]"
		h = "[FCLNS]"
"#
    }
}
pub mod _MI {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MI = "\^MI" MI_fields
    MI_fields = type_ "," message
		type_ = "[RC]"
		message = "[^,]{1,63}"
"#
    }
}
pub mod _ML {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
ML = "\^ML" ML_fields
    ML_fields = a
		a = "([1-9][0-9]{0,4}|32000)"
"#
    }
}
pub mod _MM {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MM = "\^MM" MM_fields
    MM_fields = a ("," b)?
		a = "[TPRACDFLUK]"
		b = YesNo
"#
    }
}
pub mod _MN {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MN = "\^MN" MN_fields
    MN_fields = a ("," b)?
		a = "[NYWMA V]"
		b = "(-?[0-9]{1,3}|[12][0-9]{2}|566)"
"#
    }
}
pub mod _MP {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
MP = "\^MP" MP_fields
    MP_fields = a
		a = "[DPCE SWFXM]"
"# }
}
pub mod _MT {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
MT = "\^MT" MT_fields
    MT_fields = a
		a = "[TD]"
"# }
}
pub mod _MU {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
MU = "\^MU" MU_fields
    MU_fields = a ("," b)? ("," c)?
		a = "[DIMd]"
		b = "(150|200|300)"
		c = "(150|200|300|600)"
"#
    }
}
pub mod _MW {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
MW = "\^MW" MW_fields
    MW_fields = a
		a = YesNo
"# }
}
pub mod _PA {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! {
        r#"
PA = "\^PA" PA_fields
    PA_fields = a? ("," b)? ("," c)? ("," d)?
		a = "[01]"
		b = "[01]"
		c = "[01]"
		d = "[01]"
"#
    }
}
pub mod _PF {
    use super::{common_exprs::*, _FO::x};
    abstract_parser::grammar::extended::macros::grammar! { r#"
PF = "\^PF" PF_fields
    PF_fields = n
		n = x
"# }
}
pub mod _PH {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"PH = "\^PH""# }
}
pub mod system_PH {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"PH = "~PH""# }
}
pub mod system_PL {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
PL = "~PL" PL_fields
    PL_fields = a
		a = "([0-9]{3})"
"# }
}
pub mod _PM {
    use super::common_exprs::*;
    abstract_parser::grammar::extended::macros::grammar! { r#"
PM = "\^PM" PM_fields
    PM_fields = a
		a = YesNo
"# }
}
mod common_exprs {
    pub use tmp::{colon, comma, dot, hyphen, space, underscore, YesNo, REBA};
    mod tmp {
        abstract_parser::grammar::extended::macros::grammar! {
            r#"
REBA = "[REBA]"
YesNo = "[YN]"
comma = ","
colon = ":"
dot = "\."
underscore = "_"
hyphen = "-"
space = "\s+"?
"#
        }
    }
}
