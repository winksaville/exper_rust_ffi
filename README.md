# Experiment with Rust FFI to C

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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
