use std::fs::File;
use std::path::Path;

use super::base::SourceTrait;
use super::root::SourceRoot;
use crate::error::*;

use serde::{de::DeserializeOwned, Serialize};

const PATH_CACHE: &str = "./.cache";

impl<V, Imp> SourceRoot<V, Imp> {
    pub fn load<P>(root: P, imp: Imp) -> Result<Self>
    where
        P: AsRef<Path>,
        V: DeserializeOwned,
        Imp: SourceTrait,
    {
        let root = root.as_ref();

        let path = root.join(PATH_CACHE);
        if path.exists() {
            let file = File::open(path)?;
            Ok(Self {
                data: bincode::deserialize_from(file)?,
                root: root.to_owned(),
                imp,
            })
        } else {
            Ok(Self::empty(root, imp))
        }
    }

    pub fn save(&self) -> Result<()>
    where
        V: Serialize,
    {
        let file = File::create(self.root.join(PATH_CACHE))?;
        bincode::serialize_into(file, &self.data)?;
        Ok(())
    }
}
