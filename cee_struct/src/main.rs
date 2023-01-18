//use exper_rust_ffi::examples::cee_struct::{CeeStruct, cee_struct_init_rs};
//use crate::examples::cee_struct::{CeeStruct, cee_struct_init_rs};
use cee_struct::{CeeStruct, cee_struct_init_rs};


fn main() {
    let mut cs = CeeStruct { x: 0, y: 0 };
    cee_struct_init_rs(123, 456, &mut cs);

    println!("cs: {cs:?}");
}
