#![cfg_attr(feature = "fs_try_exists", feature(fs_try_exists))]
#![cfg_attr(feature = "wasi_ext", feature(wasi_ext))]
#![cfg_attr(feature = "dir_entry_ext2", feature(dir_entry_ext2))]

mod dir_builder;
mod dir_entry;
mod file;
mod metadata;
mod open_options;
mod read_dir;

pub use dir_builder::DirBuilder;
pub use dir_entry::DirEntry;
pub use file::File;
pub use metadata::Metadata;
pub use open_options::OpenOptions;
pub use read_dir::ReadDir;

/// [`std::fs::canonicalize`]
pub fn canonicalize<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<std::path::PathBuf> {
    let path = path.as_ref();
    std::fs::canonicalize(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::copy`]
pub fn copy<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    from: P,
    to: Q,
) -> crate::io::Result<u64> {
    let from = from.as_ref();
    std::fs::copy(from, to).map_err(|e| crate::io::Error::from_with_path(e, from))
}

/// [`std::fs::create_dir`]
pub fn create_dir<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::create_dir(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::create_dir_all`]
pub fn create_dir_all<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::hard_link`]
pub fn hard_link<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    original: P,
    link: Q,
) -> crate::io::Result<()> {
    let original = original.as_ref();
    std::fs::hard_link(original, link).map_err(|e| crate::io::Error::from_with_path(e, original))
}

/// [`std::fs::metadata`]
pub fn metadata<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<std::fs::Metadata> {
    let path = path.as_ref();
    std::fs::metadata(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::read`]
pub fn read<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<Vec<u8>> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::read_dir`]
pub fn read_dir<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<std::fs::ReadDir> {
    let path = path.as_ref();
    std::fs::read_dir(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::read_link`]
pub fn read_link<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<std::path::PathBuf> {
    let path = path.as_ref();
    std::fs::read_link(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::read_to_string`]
pub fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::remove_dir`]
pub fn remove_dir<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::remove_dir(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::remove_file`]
pub fn remove_dir_all<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::remove_dir_all(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::remove_file`]
pub fn remove_file<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::remove_file(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::rename`]
pub fn rename<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    from: P,
    to: Q,
) -> crate::io::Result<()> {
    let from = from.as_ref();
    std::fs::rename(from, to).map_err(|e| crate::io::Error::from_with_path(e, from))
}

/// [`std::fs::set_permissions`]
pub fn set_permissions<P: AsRef<std::path::Path>>(
    path: P,
    perm: std::fs::Permissions,
) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::set_permissions(path, perm).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::soft_link`]
#[deprecated]
pub fn soft_link<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    original: P,
    link: Q,
) -> crate::io::Result<()> {
    let original = original.as_ref();
    #[allow(deprecated)]
    std::fs::soft_link(original, link).map_err(|e| crate::io::Error::from_with_path(e, original))
}

/// [`std::fs::symlink_metadata`]
pub fn symlink_metadata<P: AsRef<std::path::Path>>(
    path: P,
) -> crate::io::Result<std::fs::Metadata> {
    let path = path.as_ref();
    std::fs::symlink_metadata(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::write`]
pub fn write<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> crate::io::Result<()> {
    let path = path.as_ref();
    std::fs::write(path, contents).map_err(|e| crate::io::Error::from_with_path(e, path))
}

/// [`std::fs::try_exists`]
#[cfg(feature = "fs_try_exists")]
pub fn try_exists<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<bool> {
    let path = path.as_ref();
    std::fs::try_exists(path).map_err(|e| crate::io::Error::from_with_path(e, path))
}



#[derive(Debug)]
pub struct DirEntry(std::fs::DirEntry);

impl From<crate::fs::DirEntry> for std::fs::DirEntry {
    fn from(entry: crate::fs::DirEntry) -> Self {
        entry.0
    }
}

impl From<std::fs::DirEntry> for crate::fs::DirEntry {
    fn from(entry: std::fs::DirEntry) -> Self {
        Self(entry)
    }
}

impl crate::fs::DirEntry {
    pub fn file_name(&self) -> std::ffi::OsString {
        self.0.file_name()
    }

    pub fn file_type(&self) -> crate::io::Result<std::fs::FileType> {
        self.0
            .file_type()
            .map_err(|e| crate::io::Error::from_with_path(e, self.0.path()))
    }

    pub fn metadata(&self) -> crate::io::Result<std::fs::Metadata> {
        self.0
            .metadata()
            .map_err(|e| crate::io::Error::from_with_path(e, self.0.path()))
    }

    pub fn path(&self) -> std::path::PathBuf {
        self.0.path()
    }
}

#[cfg(target_os = "wasi")]
#[cfg(feature = "wasi_ext")]
impl std::os::wasi::fs::DirEntryExt for crate::fs::DirEntry {
    fn ino(&self) -> u64 {
        self.0.ino()
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirEntryExt for crate::fs::DirEntry {
    fn ino(&self) -> u64 {
        self.0.ino()
    }
}

#[cfg(unix)]
#[cfg(feature = "dir_entry_ext2")]
impl std::os::unix::fs::DirEntryExt2 for crate::fs::DirEntry {
    fn file_name_ref(&self) -> &std::ffi::OsStr {
        self.0.file_name_ref()
    }
}

#[derive(Debug)]
pub struct File {
    inner: std::fs::File,
    path: Option<std::path::PathBuf>,
}

impl From<crate::fs::File> for std::fs::File {
    fn from(file: crate::fs::File) -> Self {
        file.inner
    }
}

impl From<std::fs::File> for crate::fs::File {
    fn from(file: std::fs::File) -> Self {
        Self {
            inner: file,
            path: None,
        }
    }
}

impl crate::fs::File {
    pub fn create<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<crate::fs::File> {
        let path = path.as_ref();
        std::fs::File::create(path)
            .map(|file| crate::fs::File::from_with_path(file, path))
            .map_err(|e| crate::io::Error::from_with_path(e, path))
    }

    pub fn create_new<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<crate::fs::File> {
        let path = path.as_ref();
        std::fs::File::create_new(path)
            .map(|file| crate::fs::File::from_with_path(file, path))
            .map_err(|e| crate::io::Error::from_with_path(e, path))
    }

    pub fn metadata(&self) -> crate::io::Result<std::fs::Metadata> {
        self.inner.metadata().map_err(|e| crate::io::Error {
            inner: e,
            path: self.path.clone(),
        })
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> crate::io::Result<crate::fs::File> {
        let path = path.as_ref();
        std::fs::File::open(path)
            .map(|file| crate::fs::File::from_with_path(file, path))
            .map_err(|e| crate::io::Error::from_with_path(e, path))
    }

    pub fn options() -> crate::fs::OpenOptions {
        std::fs::OpenOptions::new().into()
    }

    pub fn try_clone(&self) -> crate::io::Result<crate::fs::File> {
        self.inner
            .try_clone()
            .map(|file| crate::fs::File {
                inner: file,
                path: self.path.clone(),
            })
            .map_err(|e| crate::io::Error {
                inner: e,
                path: self.path.clone(),
            })
    }

    pub(crate) fn from_with_path(file: std::fs::File, path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref();
        Self {
            inner: file,
            path: Some(path.to_path_buf()),
        }
    }
}
