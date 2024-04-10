#![cfg_attr(feature = "error_generic_member_access", feature(error_generic_member_access))]

pub fn var<K: AsRef<std::ffi::OsStr>>(key: K) -> Result<String, crate::env::VarError> {
    let key = key.as_ref();
    std::env::var(key).map_err(|e| crate::env::VarError{inner: e, key: key.to_os_string()})
}

#[derive(Clone, Debug, Eq, PartialEq, derive_more::Display, derive_more::Deref, derive_more::DerefMut, derive_more::AsRef, derive_more::AsMut, derive_more::Into)]
#[display(fmt = "{:?} {}", key, inner)]
pub struct VarError {
    #[deref]
    #[deref_mut]
    #[as_ref]
    #[as_mut]
    #[into]
    pub(crate) inner: std::env::VarError,
    pub(crate) key: Option<std::ffi::OsString>,
}
impl std::error::Error for self::VarError {
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