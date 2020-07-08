use std::path::{Path, PathBuf};

use crate::output::Output;

use sos_client::{Result, SourceTrait};

pub struct SimpleSourceImpl;

impl SourceTrait for SimpleSourceImpl {
    type Code = Output;

    fn compile_source(&self, source: String) -> Result<Self::Code> {
        Ok(Output(source))
    }

    fn name_to_file(&self, root: &Path, name: &str) -> PathBuf {
        root.join(name)
    }
}
