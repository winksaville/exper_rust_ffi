# A struct usable in Rust and C

For a start I searched for [rust call c function](https://www.google.com/search?q=rust+call+c+function)
and this from the [embedded rust book](https://docs.rust-embedded.org/book/interoperability/c-with-rust.html)
was the first hit and seems like a good start.

## Run

```
$ cargo run
   Compiling cc v1.0.78
   Compiling exper_rust_ffi_to_c v0.1.0 (/home/wink/prgs/rust/myrepos/exper_rust_ffi)
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/exper_rust_ffi_to_c`
cs: CeeStruct { x: 123, y: 456 }
```
