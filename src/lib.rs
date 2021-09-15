#![allow(clippy::missing_safety_doc)]

use std::ptr::NonNull;

pub struct Model {
    name: String,
}

impl Drop for Model {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn model__new(name: *const std::os::raw::c_char) -> *mut Model {
    let name = std::ffi::CStr::from_ptr(name)
        .to_str()
        .unwrap_or("invalid")
        .to_owned();
    Box::into_raw(Box::new(Model { name }))
}

#[no_mangle]
pub extern "C" fn model__init(ptr: Option<NonNull<Model>>) {
    if let Some(model) = ptr.map(|ptr| unsafe { ptr.as_ref() }) {
        println!("model: init");
    }
}
#[no_mangle]
pub extern "C" fn model__hello(ptr: Option<NonNull<Model>>) {
    if let Some(model) = ptr.map(|ptr| unsafe { ptr.as_ref() }) {
        println!("model: my name is {}", model.name);
    }
}

#[no_mangle]
pub extern "C" fn model__drop(ptr: Option<NonNull<Model>>) {
    ptr.map(NonNull::as_ptr)
        .map(|ptr| unsafe { Box::from_raw(ptr) });
}
