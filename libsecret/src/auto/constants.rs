// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use std::ffi::CStr;

pub static BACKEND_EXTENSION_POINT_NAME: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::SECRET_BACKEND_EXTENSION_POINT_NAME)
            .to_str()
            .unwrap()
    });
pub static COLLECTION_DEFAULT: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::SECRET_COLLECTION_DEFAULT)
            .to_str()
            .unwrap()
    });
pub static COLLECTION_SESSION: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::SECRET_COLLECTION_SESSION)
            .to_str()
            .unwrap()
    });
