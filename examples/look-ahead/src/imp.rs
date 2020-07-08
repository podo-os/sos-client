use std::path::PathBuf;

use crate::output::Output;

use sos_client::{Result, SourceTrait};

pub struct SimpleSourceImpl;

impl SourceTrait for SimpleSourceImpl {
    type Code = Output;
    type PrebuiltCode = String;

    fn compile_source(&self, source: Self::PrebuiltCode) -> Result<Self::Code> {
        Ok(Output(source))
    }

    fn source_file_postfix(&self) -> &[&str] {
        &[".txt"]
    }

    fn prebuild(&self, path: PathBuf, source: String) -> Option<(String, Self::PrebuiltCode)> {
        let key = path.file_name()?.to_str()?.to_string();
        Some((key, source))
    }
}
