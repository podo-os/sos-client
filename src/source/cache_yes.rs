use std::fs::File;
use std::path::Path;

use super::base::SourceTrait;
#[cfg(feature = "look-ahead")]
use super::look_ahead_yes::PrebuiltData;
use super::root::SourceRoot;
use crate::error::*;

use serde::{de::DeserializeOwned, Serialize};

const PATH_CACHE: &str = "./.cache";
#[cfg(feature = "look-ahead")]
const PATH_CACHE_PREBUILT: &str = "./.prebuilt.cache";

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait,
{
    pub fn load<P>(root: P, imp: Imp) -> Result<Self>
    where
        P: AsRef<Path>,
        V: DeserializeOwned,
    {
        let root = root.as_ref();

        let path = root.join(PATH_CACHE);
        if path.exists() {
            let file = File::open(path)?;
            Ok(Self {
                #[cfg(feature = "look-ahead")]
                prebuilt: load_prebuilt_data(root, &imp)?,
                data: bincode::deserialize_from(file)?,
                root: root.to_owned(),
                imp,
            })
        } else {
            Self::new(root, imp)
        }
    }

    pub fn save(&self) -> Result<()>
    where
        V: Serialize,
    {
        #[cfg(feature = "look-ahead")]
        {
            let file = File::create(self.root.join(PATH_CACHE_PREBUILT))?;
            bincode::serialize_into(file, &self.prebuilt)?;
        }

        {
            let file = File::create(self.root.join(PATH_CACHE))?;
            bincode::serialize_into(file, &self.data)?;
        }
        Ok(())
    }
}

#[cfg(feature = "look-ahead")]
fn load_prebuilt_data<Imp>(root: &Path, imp: &Imp) -> Result<PrebuiltData<Imp::PrebuiltCode>>
where
    Imp: SourceTrait,
{
    let path = root.join(PATH_CACHE_PREBUILT);
    if path.exists() {
        let file = File::open(path)?;
        Ok(PrebuiltData {
            inner: bincode::deserialize_from(file)?,
        })
    } else {
        Ok(PrebuiltData::load(root, imp))
    }
}
