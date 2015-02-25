#![crate_name = "libc"]
#![crate_type = "rlib"]

#![feature(no_std, core)]
#![no_std]

extern crate core;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type size_t = u32;
pub type c_int = i32;



extern {
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
}
