#[derive(Debug)]
pub struct DirBuilder(std::fs::DirBuilder);

impl From<self::DirBuilder> for std::fs::DirBuilder {
    fn from(builder: self::DirBuilder) -> Self {
        builder.0
    }
}

impl From<std::fs::DirBuilder> for self::DirBuilder {
    fn from(builder: std::fs::DirBuilder) -> Self {
        Self(builder)
    }
}

impl self::DirBuilder {
    pub fn create<P: AsRef<std::path::Path>>(&self, path: P) -> crate::io::Result<()> {
        let path = path.as_ref();
        self.0
            .create(path)
            .map_err(|e| crate::io::Error::from_with_path(e, path))
    }

    pub fn new() -> Self {
        Self(std::fs::DirBuilder::new())
    }

    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.0.recursive(recursive);
        self
    }
}

#[cfg(unix)]
impl std::os::unix::fs::DirBuilderExt for self::DirBuilder {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }
}