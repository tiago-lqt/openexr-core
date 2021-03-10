// #![allow(non_upper_case_globals)]

use std::ffi::CString;

use crate::ExrError;
use openexr_sys::*;

pub struct ExrFile {
    pub(crate) file: *mut exr_file_t,
}

impl Drop for ExrFile {
    fn drop(&mut self) {
        unsafe {
            exr_close(&mut self.file);

            self.file = std::ptr::null::<exr_file_t>() as *mut exr_file_t;
        }
    }
}

pub fn open_file(path: &str) -> Result<ExrFile, ExrError> {
    unsafe {
        let c_path = CString::new(path)?;

        let mut file: *mut exr_file_t = std::ptr::null::<exr_file_t>() as *mut exr_file_t;

        let result = exr_start_read((&mut file) as *mut *mut exr_file_t, c_path.as_ptr(), None);

        if result != 0 {
            todo!();
        }

        Ok(ExrFile { file })
    }
}
