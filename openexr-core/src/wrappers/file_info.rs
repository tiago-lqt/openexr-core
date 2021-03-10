use std::{ffi::CString, os};

use crate::{ExrError, ExrFile, StorageType};

impl ExrFile {
    pub fn parts_count(&self) -> Result<i32, ExrError> {
        unsafe {
            match openexr_sys::exr_get_part_count(self.file) {
                // TODO: Use proper error
                0 => Err(ExrError::Unknown),
                count => Ok(count),
            }
        }
    }

    pub fn has_deep(&self) -> bool {
        unsafe { openexr_sys::exr_has_deep(self.file) != 0 }
    }

    pub fn get_file_name(&self) -> Result<String, ExrError> {
        unsafe {
            let raw: *const os::raw::c_char = openexr_sys::exr_get_file_name(self.file);

            if raw == std::ptr::null() {
                return Ok(String::new());
            }

            let c_string = CString::from_raw(raw as *mut os::raw::c_char);

            match c_string.into_string() {
                Ok(string) => Ok(string),

                // TODO: Use proper error
                Err(_) => Err(ExrError::Unknown),
            }
        }
    }

    pub fn get_part_name(&self, part_index: i32) -> Result<String, ExrError> {
        unsafe {
            let raw: *const os::raw::c_char = openexr_sys::exr_get_part_name(self.file, part_index);

            if raw == std::ptr::null() {
                return Ok(String::new());
            }

            let c_string = CString::from_raw(raw as *mut os::raw::c_char);

            match c_string.into_string() {
                Ok(string) => Ok(string),

                // TODO: Use proper error
                Err(_) => Err(ExrError::Unknown),
            }
        }
    }

    pub fn get_part_storage(&self, part_index: i32) -> Result<StorageType, ExrError> {
        unsafe {
            match openexr_sys::exr_get_part_storage(self.file, part_index) {
                openexr_sys::exr_STORAGE_TYPE_t_EXR_STORAGE_SCANLINE => Ok(StorageType::Scanline),
                openexr_sys::exr_STORAGE_TYPE_t_EXR_STORAGE_TILED => Ok(StorageType::Tiled),
                openexr_sys::exr_STORAGE_TYPE_t_EXR_STORAGE_DEEP_SCANLINE => {
                    Ok(StorageType::DeepScanline)
                }
                openexr_sys::exr_STORAGE_TYPE_t_EXR_STORAGE_DEEP_TILED => {
                    Ok(StorageType::DeepTiled)
                }

                // TODO: Use proper error
                openexr_sys::exr_STORAGE_TYPE_t_EXR_STORAGE_LAST_TYPE => Err(ExrError::Unknown),

                // TODO: Use proper error
                _ => Err(ExrError::Unknown),
            }
        }
    }

    pub fn get_scanlines_per_chunk(&self, part_index: i32) -> Result<i32, ExrError> {
        unsafe {
            match openexr_sys::exr_get_scanlines_per_chunk(self.file, part_index) {
                // TODO: Use proper error
                -1 => Err(ExrError::Unknown),
                count => Ok(count),
            }
        }
    }

    pub fn get_chunk_count(&self, part_index: i32) -> Result<i32, ExrError> {
        unsafe {
            match openexr_sys::exr_get_chunk_count(self.file, part_index) {
                // TODO: Use proper error
                -1 => Err(ExrError::Unknown),
                count => Ok(count),
            }
        }
    }

    // TODO: Use better return type
    pub fn get_tile_levels(&self, part_index: i32) -> Result<(i32, i32), ExrError> {
        unsafe {
            let levelsx: i32 = -1;
            let levelsy: i32 = -1;

            match openexr_sys::exr_get_tile_levels(
                self.file,
                part_index,
                levelsx as *mut i32,
                levelsy as *mut i32,
            ) {
                openexr_sys::exr_ERROR_CODES_t_EXR_ERR_SUCCESS => Ok((levelsx, levelsy)),
                error => Err(error.into()),
            }
        }
    }

    // TODO: Use better return type
    pub fn get_tile_sizes(
        &self,
        part_index: i32,
        levelx: i32,
        levely: i32,
    ) -> Result<(i32, i32), ExrError> {
        unsafe {
            let tilew: i32 = -1;
            let tileh: i32 = -1;

            match openexr_sys::exr_get_tile_sizes(
                self.file,
                part_index,
                levelx,
                levely,
                tilew as *mut i32,
                tileh as *mut i32,
            ) {
                openexr_sys::exr_ERROR_CODES_t_EXR_ERR_SUCCESS => Ok((tilew, tileh)),
                error => Err(error.into()),
            }
        }
    }
}
