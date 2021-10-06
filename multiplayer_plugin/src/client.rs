use std::collections::VecDeque;

use bevy::prelude::*;
use shared::{network::udp_client::ComClient, ClientMessage, Command, ServerMessage, Vec2};

pub struct MultiplayerClientPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Connect,
    On,
    Off,
}

pub struct ConnectInfo {
    pub remote_addr: String,
    pub base_addr: String,
    pub base_local_port: usize,
    pub port_try_amount: usize,
}

#[derive(Default)]
pub struct MessagesToSend {
    messages: VecDeque<ClientMessage>,
}
impl MessagesToSend {
    pub fn push(&mut self, message: ClientMessage) {
        self.messages.push_back(message);
    }
}

#[derive(Default)]
pub struct MessagesToRead {
    messages: VecDeque<ServerMessage>,
}
impl MessagesToRead {
    pub fn pop(&mut self) -> Option<ServerMessage> {
        let message = self.messages.pop_back();
        self.messages.clear();
        message
    }
}
impl Plugin for MultiplayerClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // FIXME: should build on startup on "State::On" only or on message received
        let remote_addr = "127.0.0.1:8083";
        let base_addr = "127.0.0.1:".to_string();
        let base_local_port = 34255;
        let com: Option<ComClient> = None;

        app.add_state(State::Off);

        app.insert_resource(ConnectInfo {
            remote_addr: remote_addr.to_string(),
            base_addr,
            base_local_port,
            port_try_amount: 10,
        });
        app.insert_resource(com);
        app.insert_resource(MessagesToRead::default());
        app.insert_resource(MessagesToSend::default());
        app.add_system_set(
            SystemSet::on_update(State::On)
                .with_system(receive_messages.system())
                .with_system(send_messages.system()),
        );
        app.add_system_set(SystemSet::on_enter(State::Connect).with_system(connect.system()));
        app.add_system_set(SystemSet::on_enter(State::Off).with_system(disconnect.system()));
    }
}

fn connect(
    mut state_network: ResMut<bevy::prelude::State<State>>,
    connect_informations: Res<ConnectInfo>,
    mut com: ResMut<Option<ComClient>>,
) {
    if com.is_some() {
        return;
    }
    let c = {
        let mut ret = None;
        for i in 0..connect_informations.port_try_amount {
            let mut addr = connect_informations.base_addr.to_string();
            addr.push_str(&(connect_informations.base_local_port + i).to_string());
            if let Ok(com) = ComClient::connect(&addr, &connect_informations.remote_addr) {
                ret = Some(com);
                break;
            }
        }
        ret
    };
    if let Some(c) = c {
        let value = ClientMessage {
            command: Command::MoveDirection(Vec2 { x: 0.0, y: 10.0 }),
        };
        c.send(&value);
        com.replace(c);
        state_network.replace(State::On);
    } else {
        state_network.replace(State::Off);
    }
}

fn disconnect(mut com: ResMut<Option<ComClient>>) {
    *com = None;
    dbg!("disconnect");
}

fn receive_messages(
    mut state_network: ResMut<bevy::prelude::State<State>>,
    mut com_to_read: ResMut<Option<ComClient>>,
    mut messages_to_read: ResMut<MessagesToRead>,
) {
    if let Some(com_to_read) = &mut *com_to_read {
        while let Some(msg) = match com_to_read.receive() {
            Ok(msg) => Some(msg),
            Err(err) => {
                if let Some(err_io) = err.downcast_ref::<std::io::Error>() {
                    if let std::io::ErrorKind::WouldBlock = err_io.kind() {
                        return;
                    }
                }
                state_network.replace(State::Off);
                return;
            }
        } {
            messages_to_read.messages.push_back(msg);
        }
    }
}

fn send_messages(
    mut state_network: ResMut<bevy::prelude::State<State>>,
    mut com_to_send: ResMut<Option<ComClient>>,
    mut messages_to_send: ResMut<MessagesToSend>,
) {
    let mut error = false;
    if let Some(com_to_send) = &mut *com_to_send {
        for msg in messages_to_send.messages.iter() {
            error = com_to_send.send(msg).is_err();
        }
        messages_to_send.messages.clear();
    }
    if error {
        state_network.replace(State::Off);
    }
}
