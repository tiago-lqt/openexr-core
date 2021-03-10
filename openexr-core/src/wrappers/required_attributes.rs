use crate::{Boxi32, ExrFile};

impl ExrFile {
    pub fn get_display_window(&self, part_index: i32) -> Boxi32 {
        unsafe { openexr_sys::exr_get_display_window(self.file, part_index).into() }
    }

    pub fn get_data_window(&self, part_index: i32) -> Boxi32 {
        unsafe { openexr_sys::exr_get_data_window(self.file, part_index).into() }
    }
}
