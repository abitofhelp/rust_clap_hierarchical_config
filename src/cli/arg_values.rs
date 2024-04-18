// use std::path::PathBuf;
//
// use phf::phf_map;

use phf::phf_map;

#[derive(Clone, Debug)]
pub(crate) enum ArgKind {
    Bool,
    I128,
    I16,
    I32,
    I64,
    I8,
    Str,
    String,
    U128,
    U16,
    U32,
    U64,
    U8,
}

impl ArgKind {
    // pub fn try_from<String> for Arg<bool> {
    //     type Error = Box<dyn std::error::Error>;
    //
    //     fn try_from(original_value: String) -> Result<Self, Self::Error> {
    //         let v: Arg<bool> = Arg {
    //             //name: "".to_string(),
    //             original_value: original_value.to_string(),
    //             original_kind: ArgKind::String,
    //             converted_kind: ArgKind::Bool,
    //             converted_value: original_value.parse::<bool>()?,
    //         };
    //
    //         Ok(v)
    //     }
    // }
}
static ARG_KINDS: phf::Map<&'static str, ArgKind> = phf_map! {
    "bool"      => ArgKind::Bool,
    "i128"      => ArgKind::I128,
    "i16"       => ArgKind::I16,
    "i32"       => ArgKind::I32,
    "i64"       => ArgKind::I64,
    "i8"        => ArgKind::I8,
    "str"       => ArgKind::Str,
    "string"    => ArgKind::String,
    "u128"      => ArgKind::U128,
    "u16"       => ArgKind::U16,
    "u32"       => ArgKind::U32,
    "u64"       => ArgKind::U64,
    "u8"        => ArgKind::U8,

};

pub fn parse_kind(kind: &str) -> Option<ArgKind> {
    ARG_KINDS.get(kind).cloned()
}

#[derive(Debug, Clone)]
pub struct Arg<T> {
    //pub name: String,
    pub original_value: String,
    pub original_kind: ArgKind,
    pub converted_kind: ArgKind,
    pub converted_value: T,
}


