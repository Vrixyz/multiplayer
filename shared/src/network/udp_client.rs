use std::net::UdpSocket;

use anyhow::Result;
use rmp_serde::Serializer;
use serde::{de::DeserializeOwned, Serialize};

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

    pub fn receive<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut buf = [0; 1026];
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];

        let deserialized = rmp_serde::from_read_ref(&buf)?;
        Ok(deserialized)
    }

    pub fn send<T: Serialize>(&self, message: &T) -> Result<()> {
        let mut buf = Vec::new();

        message.serialize(&mut Serializer::new(&mut buf))?;
        //dbg!(&buf);
        self.socket.send(buf.as_slice())?;
        Ok(())
    }
}
