use crate::{Boxf32, Boxi32};

impl From<openexr_sys::exr_attr_box2f_t> for Boxf32 {
    fn from(other: openexr_sys::exr_attr_box2f_t) -> Self {
        Boxf32 {
            min: mint::Vector2::<f32> {
                x: other.x_min,
                y: other.y_min,
            },
            max: mint::Vector2::<f32> {
                x: other.x_max,
                y: other.y_max,
            },
        }
    }
}

impl From<openexr_sys::exr_attr_box2i_t> for Boxi32 {
    fn from(other: openexr_sys::exr_attr_box2i_t) -> Self {
        Boxi32 {
            min: mint::Vector2::<i32> {
                x: other.x_min,
                y: other.y_min,
            },
            max: mint::Vector2::<i32> {
                x: other.x_max,
                y: other.y_max,
            },
        }
    }
}
