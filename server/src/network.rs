use std::{borrow::Borrow, net::SocketAddr};
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    net::UdpSocket,
};

use rkyv::{
    archived_root,
    de::deserializers::AllocDeserializer,
    ser::{serializers::AlignedSerializer, Serializer},
    AlignedVec, Deserialize,
};
use shared::{ClientMessage, ServerMessage};

#[derive(Clone)]
pub struct Client {
    id: usize,
    socker_addr: SocketAddr,
}

pub trait MessageHandler<T> {
    fn handle(message: T);
}

pub struct Communicator {
    socket: UdpSocket,
    clients: HashMap<SocketAddr, Client>,
    next_available_id: usize,
}

impl Communicator {
    pub fn connect(addr: &str) -> Self {
        let mut socket = UdpSocket::bind(addr).expect("couldn't bind to address");

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
                        socker_addr: src.clone(),
                    })
                    .clone();
                self.next_available_id += 1;
                ret
            }
        };
        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        let archived = unsafe { archived_root::<ClientMessage>(buf.as_ref()) };

        let mut deserializer = AllocDeserializer;
        let deserialized = archived
            .deserialize(&mut deserializer)
            .expect("failed to deserialize value");
        Ok((client, deserialized))
    }

    pub fn send(&self, client: &Client, message: ServerMessage) -> std::io::Result<()> {
        let mut serializer = AlignedSerializer::new(AlignedVec::new());
        serializer
            .serialize_value(&message)
            .expect("failed to serialize value");

        let mut buf = serializer.into_inner();

        self.socket
            .send_to(&*buf, client.socker_addr.clone())
            .expect("couldn't send message");
        Ok(())
    }
}
