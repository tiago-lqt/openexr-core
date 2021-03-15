use openexr_sys as sys;

pub use sys::Error;
type Result<T, E = Error> = std::result::Result<T, E>;

use crate::attr_float_vector::FloatVector;
use crate::attr_preview::Preview;
use crate::attr_simple::*;
use crate::attr_string_vector::StringVector;
use crate::tiledesc::Tiledesc;
use crate::{File, Reader, Writer};

use std::ffi::CString;

pub enum AttributeValue {
    Unknown,
    Box2i(Box2i),
    Box2f(Box2f),
    Chlist,
    Chromaticities(Chromaticities),
    Compression(CompressionType),
    Double(f64),
    Envmap(EnvmapType),
    Float(f32),
    FloatVector(FloatVector),
    Int(i32),
    Keycode(Keycode),
    LineOrder(LineOrder),
    M33f(M33f),
    M33d(M33d),
    M44f(M44f),
    M44d(M44d),
    Preview(Preview),
    Rational(Rational),
    String(String),
    StringVector(StringVector),
    Tiledesc(Tiledesc),
    Timecode(Timecode),
    V2i(V2i),
    V2f(V2f),
    V2d(V2d),
    V3i(V3i),
    V3f(V3f),
    V3d(V3d),
    Opaque(*const u8),
}

pub struct Attribute<'a> {
    inner: *const sys::exr_attribute_t,
    file: &'a dyn File,
}

pub struct AttributeList<'a> {
    pub(crate) inner: *const sys::exr_attribute_list_t,
    pub(crate) file: &'a dyn File,
}

pub struct AttributeListMut<'a> {
    pub(crate) inner: *const sys::exr_attribute_list_t,
    pub(crate) file: &'a mut dyn File,
}

impl<'a> AttributeList<'a> {
    /// Computes the number of bytes required to store this attribute
    /// list in a file
    pub fn compute_size(&self) -> u64 {
        unsafe {
            sys::exr_attr_list_compute_size(&self.inner as *const _ as *mut _)
        }
    }
}

impl<'a> AttributeListMut<'a> {
    /// Computes the number of bytes required to store this attribute
    /// list in a file
    pub fn compute_size(&self) -> u64 {
        unsafe {
            sys::exr_attr_list_compute_size(&self.inner as *const _ as *mut _)
        }
    }
}

impl<'a> Drop for AttributeList<'a> {
    fn drop(&mut self) {
        unsafe { sys::exr_attr_list_destroy(self.inner as *mut _) }
    }
}

impl<'a> Drop for AttributeListMut<'a> {
    fn drop(&mut self) {
        unsafe { sys::exr_attr_list_destroy(self.inner as *mut _) }
    }
}

impl Reader {
    /// Returns the number of attributes for the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if `part_index` is out of range
    ///
    pub fn attr_count(&self, part_index: i32) -> Result<i32> {
        let count =
            unsafe { sys::exr_attr_count(self.inner() as *mut _, part_index) };

        if count < 0 {
            sys::exr_ERROR_CODES_t(-count as u32).ok(0i32)
        } else {
            Ok(count)
        }
    }

    /// Finds the attribute with the specified name on the specified
    /// part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if the attribute is not found or
    /// `part_index` is out of range
    ///
    pub fn attr_find_by_name(
        &self,
        part_index: i32,
        name: &str,
    ) -> Result<Attribute> {
        let inner = unsafe {
            let name = CString::new(name).unwrap();
            sys::exr_attr_find_by_name(
                self.inner() as *mut _,
                part_index,
                name.as_ptr(),
            )
        };

        if inner.is_null() {
            Err(Error::InvalidArgument)
        } else {
            Ok(Attribute { inner, file: self })
        }
    }

    /// Returns the attribute list for the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if `part_index` is out of range
    ///
    pub fn attribute_list(&self, part_index: i32) -> Result<AttributeList> {
        let inner = unsafe {
            sys::exr_get_attribute_list(self.inner() as *mut _, part_index)
        };
        if inner.is_null() {
            Err(Error::InvalidArgument)
        } else {
            Ok(AttributeList { inner, file: self })
        }
    }
}

impl Writer {
    /// Returns the attribute list for the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if `part_index` is out of range
    pub fn attribute_list(&self, part_index: i32) -> Result<AttributeList> {
        let inner = unsafe {
            sys::exr_get_attribute_list(self.inner() as *mut _, part_index)
        };
        if inner.is_null() {
            Err(Error::InvalidArgument)
        } else {
            Ok(AttributeList { inner, file: self })
        }
    }

    /// Returns the mutable attribute list for the specified part
    ///
    /// # Errors
    /// - [`Error::InvalidArgument`] if `part_index` is out of range
    pub fn attribute_list_mut(
        &mut self,
        part_index: i32,
    ) -> Result<AttributeListMut> {
        let inner = unsafe {
            sys::exr_get_attribute_list(self.inner_mut() as *mut _, part_index)
        };
        if inner.is_null() {
            Err(Error::InvalidArgument)
        } else {
            Ok(AttributeListMut { inner, file: self })
        }
    }
}
