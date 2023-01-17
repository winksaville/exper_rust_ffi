use std::ffi::c_int;

#[derive(Debug)]
#[repr(C)]
pub struct CeeStruct {
    pub x: c_int,
    pub y: c_int,
}

extern "C" {
    pub fn cee_struct_init(
        x: c_int,
        y: c_int,
        cs: *mut CeeStruct
    );
}

#[inline(always)]
pub fn cee_struct_init_rs(x: c_int, y: c_int, cs: &mut CeeStruct) {
    unsafe {
        cee_struct_init(x, y, cs);
    }
}
