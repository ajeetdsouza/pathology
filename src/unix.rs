use std::env;
use std::path::{Component, Path, PathBuf};

use crate::PathExt;

impl PathExt for Path {
    fn abs(&self) -> Result<PathBuf, ()> {
        let mut stack = Vec::new();
        let mut components = self.components().peekable();

        let current_dir;
        match components.peek() {
            Some(Component::RootDir) => stack.push(components.next().unwrap()),
            _ => {
                current_dir = env::current_dir().map_err(|_| (()))?;
                stack.extend(current_dir.components());
            }
        };

        for component in components {
            match component {
                Component::Normal(_) => stack.push(component),
                Component::CurDir => (),
                Component::ParentDir => {
                    if stack.len() > 1 {
                        let _ = stack.pop();
                    }
                }
                Component::Prefix(_) => unreachable!("Windows only"),
                Component::RootDir => unreachable!("root does not occur after first component"),
            }
        }

        Ok(stack.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::PathExt;

    #[test]
    fn test_abs() {
        let test_cases = &[
            ("/", "/"),
            ("//", "//"),
            ("///", "/"),
            ("///foo/.//bar//", "/foo/bar"),
            ("///foo/.//bar//.//..//.//baz", "/foo/baz"),
            ("///..//./foo/.//bar", "/foo/bar"),
            ("/foo/../../../bar", "/bar"),
            ("/a/b/c/../../../x/y/z", "/x/y/z"),
            ("///..//./foo/.//bar", "/foo/bar"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Path::new(input).abs().unwrap(), Path::new(expected))
        }
    }
}
