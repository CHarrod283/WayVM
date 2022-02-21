

use nix::sys::socket;
use std::os::unix::io::{RawFd};
use nix::sys::socket::MsgFlags;

pub const VMADDR_CID_ANY : u32 = 0xffffffff;
pub const VMADDR_CID_HOST : u32 = 0x0;
pub const PORT_NUM : u32 = 5623;
pub const MAX_WL_MESSAGE_SIZE : usize = 4000;
pub const MAX_CLIENTS : usize = 32;



pub fn pipe(s1: RawFd, s2: RawFd){
    let mut buffer: [u8; MAX_WL_MESSAGE_SIZE] = [0; MAX_WL_MESSAGE_SIZE];
    loop {
        let size = socket::recv(s1, &mut buffer, MsgFlags::empty()).unwrap();
        socket::send(s2, &buffer[..size], MsgFlags::empty()).unwrap();
    }
}

fn main() {
    return
}