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


fn main() {
    let mut cs = CeeStruct { x: 0, y: 0 };
    unsafe {
        cee_struct_init(123, 456, &mut cs);
    }

    println!("cs: {cs:?}");
}
