# A c libary in rust

An example of creating a library in rust and
using it from C. Based on this article at
[ultrasaurs.com](https://ultrasaurus.com/2020/01/writing-c-library-in-rust/)

Currently it is just a library created using
`cargo new --lib c_lib_in_rust`

## Test

```
wink@fwlaptop 23-01-20T22:48:04.813Z:~/prgs/rust/myrepos/exper_rust_ffi/c_lib_in_rust (main)
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (/home/wink/prgs/rust/myrepos/exper_rust_ffi/target/debug/deps/c_lib_in_rust-5b1a4d54ee66b406)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c_lib_in_rust

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
