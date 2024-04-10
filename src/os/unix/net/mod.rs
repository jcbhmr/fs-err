#[derive(Debug)]
struct UnixDatagram {
    inner: std::os::unix::net::UnixDatagram,
    user_path: Option<std::path::PathBuf>,
}
