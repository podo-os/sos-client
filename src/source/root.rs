use std::collections::BTreeMap;
use std::path::Path;
#[cfg(any(not(feature = "look-ahead"), feature = "cache"))]
use std::path::PathBuf;

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
    #[cfg(any(not(feature = "look-ahead"), feature = "cache"))]
    pub(super) root: PathBuf,

    pub(super) stack: Vec<String>,
    pub(super) data: BTreeMap<String, Option<V>>,
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
            #[cfg(any(not(feature = "look-ahead"), feature = "cache"))]
            root: root.to_owned(),
            stack: Vec::new(),
            data: BTreeMap::new(),
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
        Ok(self.data.get(key).map(|x| x.as_ref().unwrap()))
    }

    pub fn find_mut<K: AsRef<str>>(&mut self, key: K) -> Result<Option<&mut V>> {
        let key = key.as_ref();
        self.ensure_compiled(key)?;
        Ok(self.data.get_mut(key).map(|x| x.as_mut().unwrap()))
    }

    fn ensure_compiled(&mut self, key: &str) -> Result<()> {
        match self.data.get(key) {
            Some(Some(_)) => Ok(()),
            Some(None) => {
                self.stack.push(key.to_owned());
                Err(Error::Cycle(self.stack.clone()))
            }
            None => {
                // push stack
                self.stack.push(key.to_owned());
                self.data.insert(key.to_owned(), None);
                // pop stack
                if let false = self.compile(key)? {
                    self.data.remove(key);
                }
                self.stack.pop();
                Ok(())
            }
        }
    }
}
