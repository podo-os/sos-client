#[derive(Debug)]
pub enum Error {
    IO(::std::io::Error),
    #[cfg(feature = "bincode")]
    Bincode(::bincode::ErrorKind),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Self::IO(e)
    }
}

#[cfg(feature = "bincode")]
impl From<::bincode::ErrorKind> for Error {
    fn from(e: ::bincode::ErrorKind) -> Self {
        Self::Bincode(e)
    }
}

#[cfg(feature = "bincode")]
impl From<::bincode::Error> for Error {
    fn from(e: ::bincode::Error) -> Self {
        Self::Bincode(*e)
    }
}
