# crossbeam_channel across Rust FFI

I asked ChatGPT for help:

 * Question:

   * Write a program using Rust and C that allows the code to communicate via a crossbeam_channel.

 * Answer:

   * Here is an example program that demonstrates communication between Rust and C using the crossbeam_channel crate:
     ```
      // main.rs
      extern crate crossbeam_channel;
      
      use std::ffi::CString;
      use std::os::raw::c_char;
      use std::thread;
      
      extern "C" {
          fn start_c_thread(send_channel: *const crossbeam_channel::Sender<i32>);
      }
      
      fn main() {
          let (sender, receiver) = crossbeam_channel::unbounded::<i32>();
          let send_channel = CString::new(format!("{:?}", sender)).unwrap();
      
          thread::spawn(move || {
              unsafe { start_c_thread(send_channel.as_ptr() as *const _) }
          });
      
          for received in receiver {
              println!("Received {} from C", received);
          }
      }
      ```
      ```
      // main.c
      #include <stdio.h>
      #include <stdlib.h>
      #include <crossbeam_channel.h>
      
      void start_c_thread(crossbeam_channel_Sender_i32* send_channel) {
          int i = 0;
          while (1) {
              crossbeam_channel_Sender_i32_send(send_channel, i);
              printf("Sent %d from C\n", i);
              i++;
              sleep(1);
          }
      }
      
      ```
      Here, Rust's main function creates an unbounded channel and spawns a new thread that runs the C function start_c_thread. This C function sends integers to the Rust thread via the channel, and the Rust thread receives and prints these integers. Note that you need to add the crossbeam_channel crate to your Cargo.toml file and include the crossbeam_channel.h in your C file, also the c code need to be compiled as C code.

## Run

```
```
