use std::path::Path;

use super::base::SourceTrait;
use super::root::SourceRoot;
use crate::error::*;

impl<V, Imp> SourceRoot<V, Imp>
where
    Imp: SourceTrait,
{
    #[inline(always)]
    pub fn load<P>(root: P, imp: Imp) -> Result<Self>
    where
        P: AsRef<Path>,
        Imp: SourceTrait,
    {
        Self::new(root.as_ref(), imp)
    }
}
