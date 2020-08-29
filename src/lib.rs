#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

use std::path::PathBuf;

pub trait PathExt {
    fn normalize(&self) -> Result<PathBuf, ()>;
}
