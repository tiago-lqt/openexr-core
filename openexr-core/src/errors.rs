use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ExrError {
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
    ScanTileMixedapi,
    TileScanMixedapi,
    Unknown,
}

impl Display for ExrError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ExrError {}

impl From<std::ffi::NulError> for ExrError {
    fn from(_: std::ffi::NulError) -> Self {
        ExrError::InvalidArgument
    }
}

impl From<i32> for ExrError {
    fn from(value: i32) -> Self {
        #[allow(non_snake_case)]
        match value {
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_OUT_OF_MEMORY => ExrError::OutOfMemory,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_INVALID_ARGUMENT => ExrError::InvalidArgument,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_FILE_ACCESS => ExrError::FileAccess,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_FILE_BAD_HEADER => ExrError::FileBadHeader,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_NOT_OPEN_READ => ExrError::NotOpenRead,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_NOT_OPEN_WRITE => ExrError::NotOpenWrite,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_READ_IO => ExrError::ReadIo,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_WRITE_IO => ExrError::WriteIo,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_NAME_TOO_LONG => ExrError::NameTooLong,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_MISSING_REQ_ATTR => ExrError::MissingReqAttr,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_INVALID_ATTR => ExrError::InvalidAttr,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_BAD_CHUNK_DATA => ExrError::BadChunkData,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_TYPE_MISMATCH => ExrError::TypeMismatch,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_SIZE_MISMATCH => ExrError::SizeMismatch,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_SCAN_TILE_MIXEDAPI => ExrError::ScanTileMixedapi,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_TILE_SCAN_MIXEDAPI => ExrError::TileScanMixedapi,
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_SUCCESS => {
                panic!("Can not convert EXR_ERR_SUCCESS to ExrError")
            }
            openexr_sys::exr_ERROR_CODES_t_EXR_ERR_UNKNOWN | _ => ExrError::Unknown,
        }
    }
}
