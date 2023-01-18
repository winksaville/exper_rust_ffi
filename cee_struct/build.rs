fn main() {
    cc::Build::new()
        .file("src/cee_struct.c")
        .compile("cee_struct.a");
}
