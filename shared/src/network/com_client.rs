use std::net::UdpSocket;

use crate::ClientMessage;

use super::super::ServerMessage;
use rkyv::{
    archived_root,
    de::deserializers::AllocDeserializer,
    ser::{serializers::AlignedSerializer, Serializer},
    AlignedVec, Deserialize,
};

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
        let mut buf = [0; 256];
        let (amt, src) = self.socket.recv_from(&mut buf)?;

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        let archived = unsafe { archived_root::<ServerMessage>(buf) };

        let mut deserializer = AllocDeserializer;
        let deserialized = archived
            .deserialize(&mut deserializer)
            .expect("failed to deserialize value");
        Ok(deserialized)
    }

    pub fn send(&self, message: &ClientMessage) -> std::io::Result<()> {
        let mut serializer = AlignedSerializer::new(AlignedVec::new());
        serializer
            .serialize_value(message)
            .expect("failed to serialize value");

        let buf = serializer.into_inner();

        self.socket.send(&*buf).expect("couldn't send message");
        Ok(())
    }
}
