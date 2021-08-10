use std::net::UdpSocket;

use rmp_serde::Serializer;
use serde::Serialize;

use crate::ClientMessage;

use super::super::ServerMessage;

pub struct ComClient {
    socket: UdpSocket,
}

impl ComClient {
    pub fn connect(local_addr: &str, remote_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(local_addr)?;
        socket
            .set_nonblocking(true)
            .expect("Failed to enter non-blocking mode");
        socket
            .connect(remote_addr)
            .expect("Failed to connect to remote server");

        Ok(Self { socket })
    }

    pub fn receive(&mut self) -> std::io::Result<ServerMessage> {
        let mut buf = [0; 1026];
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        dbg!(amt);
        let buf = &mut buf[..amt];

        let deserialized = rmp_serde::from_read_ref(&buf).expect("failed to deserialize value");
        Ok(deserialized)
    }

    pub fn send(&self, message: &ClientMessage) -> std::io::Result<()> {
        let mut buf = Vec::new();

        message
            .serialize(&mut Serializer::new(&mut buf))
            .expect("Failed to serialize data");
        //dbg!(&buf);
        self.socket
            .send(buf.as_slice())
            .expect("couldn't send message");
        Ok(())
    }
}
