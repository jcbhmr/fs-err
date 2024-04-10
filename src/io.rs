#[derive(Debug, derive_more::Display, derive_more::Deref, derive_more::DerefMut, derive_more::AsRef, derive_more::AsMut, derive_more::Into)]
#[display(fmt = "{:?} {}", path, inner)]
pub struct Error {
    #[deref]
    #[deref_mut]
    #[as_ref]
    #[as_mut]
    #[into]
    pub(crate) inner: std::io::Error,
    pub(crate) path: std::path::PathBuf,
}
impl std::error::Error for self::Error {
    delegate::delegate! {
        to self.inner {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;
            #[allow(deprecated)]
            fn description(&self) -> &str;
            #[allow(deprecated)]
            fn cause(&self) -> Option<&dyn std::error::Error>;
            #[cfg(feature = "error_generic_member_access")]
            fn provide<'a>(&'a self, request: &mut Request<'a>);
        }
    }
}
impl crate::io::Error {
    #[cfg(feature = "raw_os_error_ty")]
    pub fn from_raw_os_error(code: RawOsError) -> std::io::Error {
        std::io::Error::from_raw_os_error(code)
    }
    #[cfg(not(feature = "raw_os_error_ty"))]
    pub fn from_raw_os_error(code: i32) -> std::io::Error {
        std::io::Error::from_raw_os_error(code)
    }
    pub fn last_os_error() -> std::io::Error {
        std::io::Error::last_os_error()
    }
    pub fn new<E>(kind: std::io::ErrorKind, error: E) -> std::io::Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        std::io::Error::new(kind, error)
    }
    pub fn other<E>(error: E) -> std::io::Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        std::io::Error::other(error).into()
    }
}

impl From<std::io::ErrorKind> for crate::io::Error {
    fn from(kind: std::io::ErrorKind) -> Self {
        std::io::Error::from(kind).into()
    }
}

impl<W> From<std::io::IntoInnerError<W>> for crate::io::Error {
    fn from(iie: std::io::IntoInnerError<W>) -> Self {
        std::io::Error::from(iie).into()
    }
}

impl From<std::ffi::NulError> for crate::io::Error {
    fn from(ne: std::ffi::NulError) -> Self {
        std::io::Error::from(ne).into()
    }
}

pub type Result<T> = std::result::Result<T, crate::io::Error>;
