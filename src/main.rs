use exper_rust_ffi_to_c::{CeeStruct, cee_struct_init_rs};


fn main() {
    let mut cs = CeeStruct { x: 0, y: 0 };
    cee_struct_init_rs(123, 456, &mut cs);

    println!("cs: {cs:?}");
}
