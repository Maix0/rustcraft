pub(crate) mod tcp_handler;
use std::io::prelude::*;
use tcp_handler::*;
fn main() {
    /*let srv =
        server::TcpServer::new("localhost:25565".to_owned()).expect("Error creating the server");
    for packet in srv.srv.incoming() {
        let mut buffer = vec![0; 100];
        println!("{:?} | {:02X?}", packet.unwrap().read(&mut buffer), buffer);
    }*/
    let test = packet::PacketTypes::VarInt;
    let mut data = [0xff, 0xff, 0xff, 0xff, 0x0f];
    assert_eq!(test.read_as_var_int(&mut data), Some(-1));
}
