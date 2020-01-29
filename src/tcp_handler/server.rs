#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::net::TcpListener;
pub(crate) struct TcpServer {
    pub(crate) ip: String,
    pub(crate) srv: TcpListener,
}

impl TcpServer {
    pub(crate) fn new(ip: String) -> std::io::Result<Self> {
        Ok(TcpServer {
            srv: TcpListener::bind(&ip)?,
            ip,
        })
    }
}

/*
fn readbyte() -> usize {
    0
}

fn readvarint() -> isize {
    let mut numread = 0;
    let mut result = 0;
    let mut read = 0;
    while (read & 0b10000000) != 0 {
        read = readbyte();
        let value = read & 0b01111111;
        result |= value << (7 * numread);

        numread += 1;
        if numread > 5 {
            panic!("VarInt is too big");
        }
    }
    return result as isize;
}*/
