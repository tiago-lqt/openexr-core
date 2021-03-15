use openexr_sys as sys;
pub use sys::Tiledesc;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileLevelMode {
    OneLevel,
    MipmapLevels,
    RipmapLevels,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileRoundMode {
    RoundDown,
    RoundUp,
}
