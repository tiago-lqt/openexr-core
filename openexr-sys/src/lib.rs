#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl exr_ERROR_CODES_t {
    pub fn ok<T>(&self, val: T) -> Result<T, Error> {
        match *self {
            exr_ERROR_CODES_t::EXR_ERR_SUCCESS => Ok(val),
            exr_ERROR_CODES_t::EXR_ERR_OUT_OF_MEMORY => Err(Error::OutOfMemory),
            exr_ERROR_CODES_t::EXR_ERR_INVALID_ARGUMENT => {
                Err(Error::InvalidArgument)
            }
            exr_ERROR_CODES_t::EXR_ERR_FILE_ACCESS => Err(Error::FileAccess),
            exr_ERROR_CODES_t::EXR_ERR_FILE_BAD_HEADER => {
                Err(Error::FileBadHeader)
            }
            exr_ERROR_CODES_t::EXR_ERR_NOT_OPEN_READ => Err(Error::NotOpenRead),
            exr_ERROR_CODES_t::EXR_ERR_NOT_OPEN_WRITE => {
                Err(Error::NotOpenWrite)
            }
            exr_ERROR_CODES_t::EXR_ERR_READ_IO => Err(Error::ReadIo),
            exr_ERROR_CODES_t::EXR_ERR_WRITE_IO => Err(Error::WriteIo),
            exr_ERROR_CODES_t::EXR_ERR_NAME_TOO_LONG => Err(Error::NameTooLong),
            exr_ERROR_CODES_t::EXR_ERR_MISSING_REQ_ATTR => {
                Err(Error::MissingReqAttr)
            }
            exr_ERROR_CODES_t::EXR_ERR_INVALID_ATTR => Err(Error::InvalidAttr),
            exr_ERROR_CODES_t::EXR_ERR_BAD_CHUNK_DATA => {
                Err(Error::BadChunkData)
            }
            exr_ERROR_CODES_t::EXR_ERR_TYPE_MISMATCH => {
                Err(Error::TypeMismatch)
            }
            exr_ERROR_CODES_t::EXR_ERR_SIZE_MISMATCH => {
                Err(Error::SizeMismatch)
            }
            exr_ERROR_CODES_t::EXR_ERR_SCAN_TILE_MIXEDAPI => {
                Err(Error::ScanTileMixedApi)
            }
            exr_ERROR_CODES_t::EXR_ERR_TILE_SCAN_MIXEDAPI => {
                Err(Error::TileScanMixedApi)
            }
            _ => Err(Error::Unknown),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error {
    OutOfMemory,
    InvalidArgument,
    FileAccess,
    FileBadHeader,
    NotOpenRead,
    NotOpenWrite,
    ReadIo,
    WriteIo,
    NameTooLong,
    MissingReqAttr,
    InvalidAttr,
    BadChunkData,
    TypeMismatch,
    SizeMismatch,
    ScanTileMixedApi,
    TileScanMixedApi,
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::OutOfMemory => write!(f, "Unable to allocate memory"),
            Error::InvalidArgument => write!(f, "Invalid argument to function"),
            Error::FileAccess => write!(
                f,
                "Unable to open file (path does not exist or permission denied"
            ),
            Error::FileBadHeader => {
                write!(f, "File is not an OpenEXR file or has a bad header")
            }
            Error::NotOpenRead => write!(f, "File not opened for read"),
            Error::NotOpenWrite => write!(f, "File not opened for write"),
            Error::ReadIo => write!(f, "Error reading from stream"),
            Error::WriteIo => write!(f, "Error writing to stream"),
            Error::NameTooLong => write!(f, "Text too long for file flags"),
            Error::MissingReqAttr => {
                write!(f, "Missing required attribute in part header")
            }
            Error::InvalidAttr => write!(f, "Invalid attribute in part header"),
            Error::BadChunkData => {
                write!(f, "Mismatch in chunk data vs programmatic value")
            }
            Error::TypeMismatch => write!(f, "Attribute type mismatch"),
            Error::SizeMismatch => {
                write!(f, "Attribute type vs. size mismatch")
            }
            Error::ScanTileMixedApi => write!(
                f,
                "Attempt to use a scanline accessor function for a tiled image"
            ),
            Error::TileScanMixedApi => write!(
                f,
                "Attempt to use a tiled accessor function for a scanline image"
            ),
            Error::Unknown => write!(f, "Unkown error code"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Chromaticities {
    pub red_x: f32,
    pub red_y: f32,
    pub green_x: f32,
    pub green_y: f32,
    pub blue_x: f32,
    pub blue_y: f32,
    pub white_x: f32,
    pub white_y: f32,
}
pub type exr_attr_chromaticities_t = Chromaticities;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Keycode {
    pub film_mfc_code: i32,
    pub film_type: i32,
    pub prefix: i32,
    pub count: i32,
    pub perf_offset: i32,
    pub perfs_per_frame: i32,
    pub perfs_per_count: i32,
}
pub type exr_attr_keycode_t = Keycode;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rational {
    pub num: i32,
    pub denom: u32,
}
pub type exr_attr_rational_t = Rational;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Timecode {
    pub time_and_flags: u32,
    pub user_data: u32,
}
pub type exr_attr_timecode_t = Timecode;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tiledesc {
    x_size: u32,
    y_size: u32,
    level_and_round: u8,
}
pub type exr_attr_tiledesc_t = Tiledesc;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        println!(">>> max image width: {}", unsafe {
            exr_get_maximum_image_width()
        });
        println!(">>> max image height: {}", unsafe {
            exr_get_maximum_image_height()
        });
    }
}
