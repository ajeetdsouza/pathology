use std::path::{Path, PathBuf};

use crate::PathExt;

impl PathExt for Path {
    fn normalize(&self) -> Result<PathBuf, ()> {
        todo!()
    }
}
