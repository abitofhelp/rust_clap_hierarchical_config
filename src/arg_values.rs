// use std::path::PathBuf;
//
// use phf::phf_map;

// #[derive(Clone)]
// pub(crate) enum ArgKind {
//     // I32,
//     // I64,
//     OptionRefPathBuf,
//     OptionRefString,
//     //RefPathBuf,
//     // RefStr,
//     // RefString,
//     // Str,
//     //RefString,
//     // U32,
//     // U64,
// }
// static ARG_KINDS: phf::Map<&'static str, ArgKind> = phf_map! {
//     "config_path" => ArgKind::OptionRefPathBuf,
//     "name" => ArgKind::OptionRefString,
// };
//
// pub fn parse_config_field_kind(id: &str) -> Option<ArgKind> {
//     ARG_KINDS.get(id).cloned()
// }
//
// #[derive(Debug, Clone)]
// pub struct ArgValue<T> {
//     pub value: T,
// }
//
// impl TryFrom<PathBuf> for ArgValue<PathBuf> {
//     type Error = &'static str;
//
//     fn try_from(original: PathBuf) -> Result<Self, Self::Error> {
//         println!("Original PathBuf: '{}'", &original.display());
//         Ok(ArgValue {
//             value: original,
//         })
//     }
// }
//
// impl TryFrom<Option<PathBuf>> for ArgValue<Option<PathBuf>> {
//     type Error = &'static str;
//
//     fn try_from(original: Option<PathBuf>) -> Result<Self, Self::Error> {
//         println!("Original PathBuf: '{}'", &original.clone().unwrap().display());
//         Ok(ArgValue {
//             value: original,
//         })
//     }
// }
//
// impl TryFrom<String> for ArgValue<String> {
//     type Error = &'static str;
//
//     fn try_from(original: String) -> Result<Self, Self::Error> {
//         println!("Original String: '{}'", &original);
//         Ok(ArgValue {
//             value: original,
//         })
//     }
// }
//
// impl TryFrom<Option<String>> for ArgValue<Option<String>> {
//     type Error = &'static str;
//
//     fn try_from(original: Option<String>) -> Result<Self, Self::Error> {
//         println!("Original String: '{}'", &original.clone().unwrap());
//         Ok(ArgValue {
//             value: original,
//         })
//     }
// }
