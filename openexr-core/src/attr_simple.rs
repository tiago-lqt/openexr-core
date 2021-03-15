use openexr_sys as sys;

pub use sys::{Chromaticities, Keycode, Rational, Timecode};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CompressionType {
    None = 0,
    RLE = 1,
    ZIPS = 2,
    ZIP = 3,
    PIZ = 4,
    PXR24 = 5,
    B44 = 6,
    B44A = 7,
    DWAA = 8,
    DWAB = 9,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EnvmapType {
    Latlong = 0,
    Cube,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LineOrder {
    IncreasingY = 0,
    DecreasingY,
    RandomY,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StorageType {
    Scanline = 0,
    Tiled,
    DeepScanline,
    DeepTiled,
}

cfg_if::cfg_if! {
    if #[cfg(feature="cgmath")] {
        pub type V2i = cgmath::Vector2<i32>;
        pub type V2f = cgmath::Vector2<f32>;
        pub type V2d = cgmath::Vector2<f64>;
        pub type V3i = cgmath::Vector3<i32>;
        pub type V3f = cgmath::Vector3<f32>;
        pub type V3d = cgmath::Vector3<f64>;

        #[repr(C)]
        pub struct Box2i {
            pub min: V2i,
            pub max: V2i,
        }

        #[repr(C)]
        pub struct Box2f {
            pub min: V2f,
            pub max: V2f,
        }

        pub type M33f = cgmath::Matrix3<f32>;
        pub type M33d = cgmath::Matrix3<f64>;

        pub type M44f = cgmath::Matrix4<f32>;
        pub type M44d = cgmath::Matrix4<f64>;

    } else if #[cfg(feature="nalgebra-glm")] {
        pub type V2i = nalgebra_glm::IVec2;
        pub type V2f = nalgebra_glm::Vec2;
        pub type V2d = nalgebra_glm::DVec2;
        pub type V3i = nalgebra_glm::IVec3;
        pub type V3d = nalgebra_glm::DVec3;

        #[repr(C)]
        pub struct Box2i {
            pub min: V2i,
            pub max: V2i,
        }

        #[repr(C)]
        pub struct Box2f {
            pub min: V2f,
            pub max: V2f,
        }

        pub type M33f = nalgebra_glm::Mat3x3;
        pub type M33d = nalgebra_glm::DMat3x3;
        pub type M44f = nalgebra_glm::Mat4x4;
        pub type M44d = nalgebra_glm::DMat4x4;

    } else {
        // internal types

        #[repr(C)]
        pub struct V2i {
            pub x: i32,
            pub y: i32,
        }

        #[repr(C)]
        pub struct V2f {
            pub x: f32,
            pub y: f32,
        }

        #[repr(C)]
        pub struct V2d {
            pub x: f64,
            pub y: f64,
        }

        #[repr(C)]
        pub struct V3i {
            pub x: i32,
            pub y: i32,
            pub z: i32,
        }

        #[repr(C)]
        pub struct V3f {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[repr(C)]
        pub struct V3d {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        #[repr(C)]
        pub struct Box2i {
            pub min: V2i,
            pub max: V2i,
        }

        #[repr(C)]
        pub struct Box2f {
            pub min: V2f,
            pub max: V2f,
        }

        #[repr(C)]
        pub struct M33f {
            pub m: [f32; 9]
        }

        #[repr(C)]
        pub struct M33d {
            pub m: [f64; 9]
        }

        #[repr(C)]
        pub struct M44f {
            m: [f32; 16]
        }

        #[repr(C)]
        pub struct M44d {
            m: [f64; 16]
        }
    }

}
