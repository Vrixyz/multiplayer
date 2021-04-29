use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    net::UdpSocket,
};
use std::{hash::Hash, net::SocketAddr};

use super::super::{ClientMessage, ServerMessage};
use rkyv::{
    archived_root,
    de::deserializers::AllocDeserializer,
    ser::{serializers::AlignedSerializer, Serializer},
    AlignedVec, Deserialize,
};

#[derive(Clone, Debug)]
pub struct Client {
    pub id: usize,
    socker_addr: SocketAddr,
}

pub struct ComServer {
    socket: UdpSocket,
    clients: HashMap<SocketAddr, Client>,
    next_available_id: usize,
}

impl ComServer {
    pub fn clients_iter(&self) -> std::collections::hash_map::Iter<SocketAddr, Client> {
        self.clients.iter()
    }
}

impl ComServer {
    pub fn bind(addr: &str) -> Self {
        let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
        socket
            .set_nonblocking(true)
            .expect("Failed to enter non-blocking mode");
        Self {
            socket,
            clients: HashMap::new(),
            next_available_id: usize::MIN,
        }
    }

    pub fn receive(&mut self) -> std::io::Result<(Client, ClientMessage)> {
        let mut buf = [0; 256];
        let (amt, src) = self.socket.recv_from(&mut buf)?;

        let client = match self.clients.entry(src) {
            Occupied(e) => e.get().clone(),
            Vacant(e) => {
                let ret = e
                    .insert(Client {
                        id: self.next_available_id,
                        socker_addr: src,
                    })
                    .clone();
                self.next_available_id += 1;
                ret
            }
        };
        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        let archived = unsafe { archived_root::<ClientMessage>(buf) };

        let mut deserializer = AllocDeserializer;
        let deserialized = archived
            .deserialize(&mut deserializer)
            .expect("failed to deserialize value");
        Ok((client, deserialized))
    }

    pub fn send(&self, client: &Client, message: &ServerMessage) -> std::io::Result<()> {
        let mut serializer = AlignedSerializer::new(AlignedVec::new());
        serializer
            .serialize_value(message)
            .expect("failed to serialize value");

        let buf = serializer.into_inner();

        self.socket
            .send_to(&*buf, client.socker_addr)
            .expect("couldn't send message");
        Ok(())
    }
}
