use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}
#[derive(Debug)]
pub struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    pub fn new(path: &str) -> Result<DirectoryIterator, String> {
        let path = CString::new(path).map_err(|err| format!("{err}"))?;
        let dir = unsafe { ffi::opendir(path.as_ptr()) };

        if dir.is_null() {
            Err(format!("Error opening {:?}", dir))
        } else {
            Ok(DirectoryIterator { path, dir })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // raw pointer
        let dirent = unsafe { ffi::readdir(self.dir) };

        if dirent.is_null() {
            None
        } else {
            let d_name = unsafe { CStr::from_ptr((*dirent).d_name.as_ptr()) };
            let osstr = OsStr::from_bytes(d_name.to_bytes());
            // TODO it's blank?
            println!("{:?}", osstr);
            Some(osstr.to_owned())
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        if !self.dir.is_null() {
            let code = unsafe { ffi::closedir(self.dir) };
            if code != 0 {
                panic!("Failed to close directory at path {:?}", self.path);
            }
        }
    }
}
