extern crate crossbeam_channel;

use std::ffi::CString;
//use std::os::raw::c_char;
use std::thread;

use crossbeam_channel::{Receiver, Sender};

//extern "C" {
//    fn start_c_thread(send_channel: *const crossbeam_channel::Sender<i32>);
//}

pub struct ChannelStruct<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> ChannelStruct<T> {
    fn send_value(&mut self, v: T) {
    }

    fn recvive_value(&mut self, v: T) {
    }
}

#
pub extern "C"

fn main() {
    let (tx, rx) = crossbeam_channel::unbounded::<i32>();

    let cs = ChannelStruct {
        tx ,
        rx,
    }
    #[no_mangle]
    let send_via_channe = (move |v: i32| {
        sender.send(v)
    });

    thread::spawn(move || {
        unsafe { start_c_thread(send_channel.as_ptr() as *const _) }
    });

    for received in receiver {
        println!("Received {} from C", received);
    }
}
