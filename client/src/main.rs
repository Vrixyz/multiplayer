use std::net::UdpSocket;
use std::{thread, time};

use shared::Command;
use shared::{ClientMessage, Vec2};

use rkyv::{
    ser::{serializers::AlignedSerializer, Serializer},
    AlignedVec,
};

fn main() -> Result<(), std::io::Error> {
    let mut socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    socket
        .connect("127.0.0.1:8080")
        .expect("connect function failed");

    let value = ClientMessage {
        command: Command::Move(Vec2 { x: 42.0, y: 10.0 }),
    };
    loop {
        let mut serializer = AlignedSerializer::new(AlignedVec::new());
        serializer
            .serialize_value(&value)
            .expect("failed to serialize value");

        let mut buf = serializer.into_inner();
        socket.send(&*buf).expect("couldn't send message");

        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        println!("{}{}{}", buf[0], buf[1], buf[2]);
        let ten_millis = time::Duration::from_millis(500);
        thread::sleep(ten_millis);
    }
}
