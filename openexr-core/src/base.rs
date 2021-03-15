use openexr_sys as sys;

pub use sys::Error;
type Result<T, E = Error> = std::result::Result<T, E>;

use std::ffi::CStr;

use crate::StorageType;

/// File contains the shared, read-only operations supported by both Reader
/// and Writer
pub trait File {
    fn inner(&self) -> *const sys::exr_file_t;

    ///////////////////////////////////////////////////////////////////////////
    // exr_file_attr.h

    /// Returns how many attributes exist in the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if the part is out of range
    fn attr_count(&self, part_index: i32) -> Result<i32> {
        let result =
            unsafe { sys::exr_attr_count(self.inner() as *mut _, part_index) };
        if result < 0 {
            Err(Error::InvalidArgument)
        } else {
            Ok(result)
        }
    }

    // fn attr_find_by_name(&self, part_index: i32, name: &str) -> Option<Attribute

    ///////////////////////////////////////////////////////////////////////////
    // exr_file_info.h

    /// Returns either the raw file name or the provided stream name depending
    /// on which creation routine was used
    ///
    fn file_name(&self) -> String {
        unsafe {
            let filename = sys::exr_get_file_name(self.inner() as *mut _);
            CStr::from_ptr(filename).to_string_lossy().to_string()
        }
    }

    /// Query how many parts are in the file
    ///
    fn part_count(&self) -> Result<i32> {
        unsafe {
            let count = sys::exr_get_part_count(self.inner() as *mut _);

            if count != 0 {
                Ok(count)
            } else {
                Err(Error::Unknown)
            }
        }
    }

    /// Query whether the file has deep in at least one of its constituent parts
    ///
    fn has_deep(&self) -> bool {
        unsafe { sys::exr_has_deep(self.inner() as *mut _) != 0 }
    }

    /// Query the part name for the specified part.
    ///
    /// # Errors
    /// - [`None`], if this is a single-part file and a name has not been set, or
    /// if the part index is out of range
    ///
    fn part_name(&self, part: i32) -> Option<String> {
        unsafe {
            let part_name =
                sys::exr_get_part_name(self.inner() as *mut _, part);
            if part_name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(part_name).to_string_lossy().to_string())
            }
        }
    }

    /// Query the storage type for the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if the part is out of range
    ///
    fn part_storage(&self, part: i32) -> Result<StorageType> {
        let st =
            unsafe { sys::exr_get_part_storage(self.inner() as *mut _, part) };
        match st {
            sys::exr_STORAGE_TYPE_t::EXR_STORAGE_SCANLINE => {
                Ok(StorageType::Scanline)
            }
            sys::exr_STORAGE_TYPE_t::EXR_STORAGE_TILED => {
                Ok(StorageType::Tiled)
            }
            sys::exr_STORAGE_TYPE_t::EXR_STORAGE_DEEP_SCANLINE => {
                Ok(StorageType::DeepScanline)
            }
            sys::exr_STORAGE_TYPE_t::EXR_STORAGE_DEEP_TILED => {
                Ok(StorageType::DeepTiled)
            }
            _ => Err(Error::InvalidArgument),
        }
    }

    /// Query the storage type for the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if the part index is out of range
    /// - [`Error::BadChunkData`] if tile data does not exist
    /// - [`Error::TileScanMixedApi`] if the file is not tiled
    ///
    fn tile_levels(&self, part: i32) -> Result<(i32, i32)> {
        let mut levelsx = 0i32;
        let mut levelsy = 0i32;
        unsafe {
            sys::exr_ERROR_CODES_t(sys::exr_get_tile_levels(
                self.inner() as *mut _,
                part,
                &mut levelsx,
                &mut levelsy,
            ) as u32)
            .ok((levelsx, levelsy))
        }
    }

    /// Query the tile size for a particular level in a particular part
    ///
    /// # Returns
    /// - On success, the width and height of the specified part level as a tuple
    /// - [`Error::InvalidArgument`] if the part index, levelx or levely are out of range
    /// - [`Error::BadChunkData`] if tile data does not exist
    /// - [`Error::TileScanMixedApi`] if the file is not tiled
    ///
    fn tile_sizes(
        &self,
        part_index: i32,
        levelx: i32,
        levely: i32,
    ) -> Result<(i32, i32)> {
        let mut tilew = 0i32;
        let mut tileh = 0i32;
        unsafe {
            sys::exr_ERROR_CODES_t(sys::exr_get_tile_sizes(
                self.inner() as *mut _,
                part_index,
                levelx,
                levely,
                &mut tilew,
                &mut tileh,
            ) as u32)
            .ok((tilew, tileh))
        }
    }

    /// Return the number of chunks contained in this part of the file
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if the part index is out of range
    ///
    fn chunk_count(&self, part_index: i32) -> Result<i32> {
        let count = unsafe {
            sys::exr_get_chunk_count(self.inner() as *mut _, part_index)
        };

        if count < 0 {
            Err(Error::InvalidArgument)
        } else {
            Ok(count)
        }
    }

    /// Return the number of scanlines per chunk for this part of the file
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if the part index is out of range
    ///
    fn scanlines_per_chunk(&self, part_index: i32) -> Result<i32> {
        let count = unsafe {
            sys::exr_get_scanlines_per_chunk(self.inner() as *mut _, part_index)
        };

        if count < 0 {
            Err(Error::InvalidArgument)
        } else {
            Ok(count)
        }
    }
}

pub fn get_library_version() -> (i32, i32, i32, String) {
    let mut maj = 0i32;
    let mut min = 0i32;
    let mut patch = 0i32;
    let mut extra = std::ptr::null();

    unsafe {
        sys::exr_get_library_version(
            &mut maj, &mut min, &mut patch, &mut extra,
        );

        (
            maj,
            min,
            patch,
            // Even if no extra version is not specified in the build,
            // the extra pointer always points to a valid (empty) string
            std::ffi::CStr::from_ptr(extra)
                .to_string_lossy()
                .into_owned(),
        )
    }
}
