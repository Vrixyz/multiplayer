use std::net::SocketAddr;
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    net::UdpSocket,
};

use rmp_serde::Serializer;
use serde::{de::DeserializeOwned, Serialize};

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

    pub fn receive<T: DeserializeOwned>(&mut self) -> std::io::Result<(Client, T)> {
        let mut buf = [0; 1056];
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        dbg!("received: {}", amt);
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
        let deserialized = rmp_serde::from_read_ref(&buf).expect("failed to deserialize value");
        Ok((client, deserialized))
    }

    pub fn send<T: Serialize>(&self, client: &Client, message: &T) -> std::io::Result<()> {
        let mut buf = Vec::new();
        message
            .serialize(&mut Serializer::new(&mut buf))
            .expect("Failed to serialize data");

        self.socket
            .send_to(buf.as_slice(), client.socker_addr)
            .expect("couldn't send message");
        Ok(())
    }
}
