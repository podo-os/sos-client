use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::base::SourceTrait;
use super::root::SourceRoot;
use crate::error::*;

use glob::glob;
use itertools::Itertools;

#[cfg_attr(feature = "cache", derive(serde::Serialize))]
pub struct PrebuiltData<D> {
    pub(super) inner: HashMap<String, D>,
}

impl<
        #[cfg(feature = "cache")] D: serde::Serialize + serde::de::DeserializeOwned,
        #[cfg(not(feature = "cache"))] D,
    > PrebuiltData<D>
{
    pub fn load<P, Imp>(root: P, imp: &Imp) -> Self
    where
        P: AsRef<Path>,
        Imp: SourceTrait<PrebuiltCode = D>,
    {
        let root = root.as_ref();
        let inner = imp
            .source_file_postfix()
            .iter()
            .map(|pf| {
                glob(&format!("{}/**/*{}", root.display(), pf))
                    .unwrap()
                    .into_iter()
                    .filter_map(|v| v.ok())
                    .filter_map(|path| {
                        let source = fs::read_to_string(&path).ok()?;
                        imp.prebuild(path, source)
                    })
                    .collect_vec()
            })
            .flatten()
            .collect();

        Self { inner }
    }
}

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait<Code = V>,
{
    pub(super) fn compile(&mut self, key: &str) -> Result<bool> {
        match self.prebuilt.inner.remove(key) {
            Some(source) => {
                let code = self.imp.compile_source(source)?;

                self.data.insert(key.to_owned(), Some(code));
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
