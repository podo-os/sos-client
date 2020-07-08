use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::base::SourceTrait;
#[cfg(feature = "look-ahead")]
use super::look_ahead_yes::PrebuiltData;
use crate::error::*;

pub struct SourceRoot<V, Imp>
where
    Imp: SourceTrait,
{
    #[cfg(feature = "look-ahead")]
    pub(super) prebuilt: PrebuiltData<Imp::PrebuiltCode>,

    pub(super) data: BTreeMap<String, V>,
    pub(super) root: PathBuf,
    pub(super) imp: Imp,
}

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait,
{
    pub(super) fn new(root: &Path, imp: Imp) -> Result<Self> {
        Ok(Self {
            #[cfg(feature = "look-ahead")]
            prebuilt: PrebuiltData::load(root, &imp),
            data: BTreeMap::new(),
            root: root.to_owned(),
            imp,
        })
    }
}

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait<Code = V>,
{
    pub fn find<'a, K: AsRef<str>>(&'a mut self, key: K) -> Result<Option<&'a V>> {
        let key = key.as_ref();
        self.ensure_compiled(key)?;
        Ok(self.data.get(key))
    }

    pub fn find_mut<K: AsRef<str>>(&mut self, key: K) -> Result<Option<&mut V>> {
        let key = key.as_ref();
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
}
