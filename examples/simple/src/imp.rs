use std::path::{Path, PathBuf};

use crate::output::Output;

use sos_client::{SourceTrait, Result};

pub struct SimpleSourceImpl;

impl SourceTrait for SimpleSourceImpl {
    type Code = Output;

    fn compile_source(&self, source: &str) -> Result<Self::Code> {
        Ok(Output(source.to_owned()))
    }

    fn name_to_file(&self, root: &Path, name: &str) -> PathBuf {
        root.join(name)
    }
}
