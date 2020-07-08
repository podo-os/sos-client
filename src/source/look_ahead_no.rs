use std::fs;

use super::base::SourceTrait;
use super::root::SourceRoot;
use crate::error::*;

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait<Code = V>,
{
    pub(super) fn compile(&mut self, key: &str) -> Result<()> {
        let path = self.imp.name_to_file(&self.root, key);
        if !path.is_file() {
            return Ok(());
        }

        let source = fs::read_to_string(&path)?;
        let code = self.imp.compile_source(source)?;

        self.data.insert(key.to_owned(), code);
        Ok(())
    }
}
