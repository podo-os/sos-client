use std::path::{Path, PathBuf};

use crate::error::Result;

pub trait SourceTrait {
    type Code;

    fn compile_source(&self, source: &str) -> Result<Self::Code>;

    fn name_to_file(&self, root: &Path, name: &str) -> PathBuf;
}
