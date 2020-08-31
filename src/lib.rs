#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

use std::path::PathBuf;

pub trait PathExt {
    /// Returns the absolute form of a path with all intermediate components
    /// normalized. Does not check if the path exists or resolve symbolic
    /// links.
    ///
    /// # Platform-specific behavior
    ///
    /// This function corresponds to the `realpath -ms` command on Unix and the
    /// `GetFullPathNameW` function on Windows. Empty paths are considered
    /// equivalent to `"."`.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * TODO
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    ///
    /// use pathology::PathExt;
    ///
    /// fn main() {
    ///     let path = Path::new("../a/../foo.txt").abs().unwrap();
    /// }
    /// ```
    fn abs(&self) -> Result<PathBuf, ()>;
}
