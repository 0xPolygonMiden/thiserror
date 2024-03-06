#[cfg(feature = "std")]
use ref_cast::RefCast;
#[cfg(feature = "std")]
use std::fmt::Display;
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};
#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
#[error("failed to read '{file}'")]
struct StructPathBuf {
    file: PathBuf,
}

#[cfg(feature = "std")]
#[derive(Error, Debug, RefCast)]
#[repr(C)]
#[error("failed to read '{file}'")]
struct StructPath {
    file: Path,
}

#[cfg(feature = "std")]
#[derive(Error, Debug)]
enum EnumPathBuf {
    #[error("failed to read '{0}'")]
    Read(PathBuf),
}

#[cfg(feature = "std")]
fn assert<T: Display>(expected: &str, value: T) {
    assert_eq!(expected, value.to_string());
}

#[cfg(feature = "std")]
#[test]
fn test_display() {
    let path = Path::new("/thiserror");
    let file = path.to_owned();
    assert("failed to read '/thiserror'", StructPathBuf { file });
    let file = path.to_owned();
    assert("failed to read '/thiserror'", EnumPathBuf::Read(file));
    assert("failed to read '/thiserror'", StructPath::ref_cast(path));
}
