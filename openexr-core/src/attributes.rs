#[derive(Debug, Copy, Clone)]
pub struct Boxf32 {
    pub min: mint::Vector2<f32>,
    pub max: mint::Vector2<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct Boxi32 {
    pub min: mint::Vector2<i32>,
    pub max: mint::Vector2<i32>,
}

#[derive(Debug, Copy, Clone)]
pub enum StorageType {
    Scanline = 0,
    Tiled,
    DeepScanline,
    DeepTiled,
}
