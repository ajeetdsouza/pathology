use winapi::shared::minwindef::DWORD;
// use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::GetFullPathNameW;
use winapi::um::winnt::WCHAR;

use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::ptr;

use crate::PathExt;

impl PathExt for Path {
    #[allow(non_snake_case)]
    fn normalize(&self) -> Result<PathBuf, ()> {
        let lpFileName: Vec<WCHAR> = self
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(b'\0' as _))
            .collect();

        let nBufferLength: DWORD =
            unsafe { GetFullPathNameW(lpFileName.as_ptr(), 0, ptr::null_mut(), ptr::null_mut()) };

        if nBufferLength == 0 {
            return Err(());
        }

        let mut lpBuffer = vec![0; nBufferLength as _];

        let lpBufferSize: DWORD = unsafe {
            GetFullPathNameW(
                lpFileName.as_ptr(),
                nBufferLength,
                lpBuffer.as_mut_ptr(),
                ptr::null_mut(),
            )
        };

        if lpBufferSize == 0 {
            return Err(());
        }

        // Convert drive letter to uppercase
        if lpBuffer.get(1) == Some(&(b':' as _)) {
            lpBuffer[0].make_ascii_uppercase();
        }

        Ok(OsString::from_wide(&lpBuffer[..lpBufferSize as _]).into())
    }
}

trait Ascii {
    fn is_ascii_lowercase(&self) -> bool;
    fn make_ascii_uppercase(&mut self);
    fn to_ascii_uppercase(&self) -> Self;
}

impl Ascii for u16 {
    fn is_ascii_lowercase(&self) -> bool {
        const LOWER_A: u16 = b'a' as _;
        const LOWER_Z: u16 = b'z' as _;

        matches!(*self, LOWER_A..=LOWER_Z)
    }

    fn make_ascii_uppercase(&mut self) {
        *self = self.to_ascii_uppercase();
    }

    fn to_ascii_uppercase(&self) -> Self {
        // Unset the fifth bit if this is a lowercase letter
        *self & !((self.is_ascii_lowercase() as u16) << 5)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::PathExt;

    #[test]
    fn test_normalize() {
        let test_cases = &[
            (r"C:///A//B", r"C:\A\B"),
            (r"D:///A/./B", r"D:\A\B"),
            (r"e:///A/foo/../B", r"e:\A\B"),
            (r"c:/", r"c:\"),
            (r"c:/../../..", r"c:\"),
            (r"C:////a/b", r"C:\a\b"),
            (r"//machine/share//a/b", r"\\machine\share\a\b"),
            (r"\\.\NUL", r"\\.\NUL"),
            (r"\\?\D:/XY\Z", r"\\?\D:\XY\Z"),
            (r"C:\.", r"C:\"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Path::new(input).normalize().unwrap(), Path::new(expected))
        }
    }
}
