use std::env;
use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use crate::PathExt;

impl PathExt for Path {
    fn normalize(&self) -> Result<PathBuf, ()> {
        let path = self.as_os_str().as_bytes();
        if path.is_empty() {
            return Err(()); // ENOENT: no such file or directory
        }

        let mut result;

        if !is_absolute(path) {
            let current_dir = env::current_dir().map_err(|_| (()))?;
            result = current_dir.as_os_str().as_bytes().to_vec();
        } else {
            result = vec![SLASH];
        }

        for component in path.split(is_separator) {
            match component {
                b"" | b"." => (),
                b".." => {
                    if let Some(idx) = result.iter().rposition(is_separator) {
                        result.truncate(idx.max(1))
                    }
                }
                _ => {
                    if result.last() != Some(&SLASH) {
                        result.push(SLASH);
                    }
                    result.extend_from_slice(component);
                }
            }
        }

        Ok(OsString::from_vec(result).into())
    }
}

const SLASH: u8 = b'/';

fn is_absolute(path: &[u8]) -> bool {
    path.first() == Some(&SLASH)
}

fn is_separator(b: &u8) -> bool {
    b == &SLASH
}
