use openexr_sys as sys;

pub use sys::Error;
type Result<T, E = Error> = std::result::Result<T, E>;

use std::ffi::CString;

use crate::File;

/// Reader is a File opened for reading by start_read(). It is safe to call
/// any methods on Reader concurrently.
pub struct Reader {
    file: *const sys::exr_file_t,
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl File for Reader {
    fn inner(&self) -> *const sys::exr_file_t {
        self.file
    }
}

pub fn start_read(filename: &str) -> Result<Reader> {
    let filename = CString::new(filename).unwrap();
    let mut file = std::ptr::null_mut();
    unsafe {
        sys::exr_ERROR_CODES_t(sys::exr_start_read(
            &mut file,
            filename.as_ptr(),
            None,
        ) as u32)
        .ok(Reader { file })
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        let _ = unsafe { sys::exr_close(self.inner() as *mut _) };
    }
}

pub struct Writer {
    file: *mut sys::exr_file_t,
}

impl File for Writer {
    fn inner(&self) -> *const sys::exr_file_t {
        self.file
    }
}

impl Writer {
    pub(crate) fn inner_mut(&mut self) -> *mut sys::exr_file_t {
        self.file
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let _ = unsafe { sys::exr_close(self.inner_mut() as *mut _) };
    }
}
