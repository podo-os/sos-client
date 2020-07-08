mod base;
mod root;

#[cfg(not(feature = "look-ahead"))]
mod look_ahead_no;
#[cfg(feature = "look-ahead")]
mod look_ahead_yes;

#[cfg(not(feature = "cache"))]
mod cache_no;
#[cfg(feature = "cache")]
mod cache_yes;

pub use self::base::SourceTrait;
pub use self::root::SourceRoot;
