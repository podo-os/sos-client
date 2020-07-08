#[cfg(not(feature = "look-ahead"))]
use std::path::Path;
use std::path::PathBuf;

use crate::error::Result;

pub trait SourceTrait {
    type Code;

    #[cfg(all(feature = "look-ahead", not(feature = "cache")))]
    type PrebuiltCode;
    #[cfg(all(feature = "look-ahead", feature = "cache"))]
    type PrebuiltCode: serde::Serialize + serde::de::DeserializeOwned;

    #[cfg(feature = "look-ahead")]
    fn compile_source(&self, source: Self::PrebuiltCode) -> Result<Self::Code>;
    #[cfg(not(feature = "look-ahead"))]
    fn compile_source(&self, source: String) -> Result<Self::Code>;

    #[cfg(feature = "look-ahead")]
    fn source_file_postfix(&self) -> &[&str];

    #[cfg(feature = "look-ahead")]
    fn prebuild(&self, path: PathBuf, source: String) -> Option<(String, Self::PrebuiltCode)>;

    #[cfg(not(feature = "look-ahead"))]
    fn name_to_file(&self, root: &Path, name: &str) -> PathBuf;
}
