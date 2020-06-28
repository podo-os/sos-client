use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::base::SourceTrait;
use crate::error::*;

pub struct SourceRoot<V, Imp> {
    pub(super) data: BTreeMap<String, V>,
    pub(super) root: PathBuf,
    pub(super) imp: Imp,
}

impl<V, Imp> SourceRoot<V, Imp> {
    pub(super) fn empty(root: &Path, imp: Imp) -> Self {
        Self {
            data: BTreeMap::new(),
            root: root.to_owned(),
            imp,
        }
    }
}

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait<Code = V>,
{
    pub fn find<'a>(&'a mut self, key: &str) -> Result<Option<&'a V>> {
        self.ensure_compiled(key)?;
        Ok(self.data.get(key))
    }

    pub fn find_mut(&mut self, key: &str) -> Result<Option<&mut V>> {
        self.ensure_compiled(key)?;
        Ok(self.data.get_mut(key))
    }

    fn ensure_compiled(&mut self, key: &str) -> Result<()> {
        if self.data.contains_key(key) {
            Ok(())
        } else {
            self.compile(key)
        }
    }

    fn compile(&mut self, key: &str) -> Result<()> {
        let path = self.imp.name_to_file(&self.root, key);
        if !path.is_file() {
            return Ok(());
        }

        let source = fs::read_to_string(&path)?;
        let code = self.imp.compile_source(&source)?;

        self.data.insert(key.to_owned(), code);
        Ok(())
    }
}
