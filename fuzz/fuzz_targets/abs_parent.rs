#![no_main]
use libfuzzer_sys::fuzz_target;

use std::path::{Path, PathBuf};

use pathology::PathExt;

#[cfg(unix)]
fuzz_target!(|dir: String| {
    if dir.contains('/') {
        return;
    }

    let path_got = PathBuf::from(format!("/foo/{}/..", dir)).abs().unwrap();
    let path_exp = Path::new("/foo");

    assert_eq!(path_got, path_exp);
});

#[cfg(windows)]
fuzz_target!(|dir: String| {
    if dir.contains('\\') {
        return;
    }

    let path_got = PathBuf::from(format!(r"C:\foo\{}\..", dir)).abs().unwrap();
    let path_exp = Path::new(r"C:\foo");

    assert_eq!(path_got, path_exp);
});
