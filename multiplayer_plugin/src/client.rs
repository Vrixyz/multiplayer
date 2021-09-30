use std::collections::VecDeque;

use bevy::prelude::*;
use shared::{network::udp_client::ComClient, ClientMessage, Command, ServerMessage, Vec2};

pub struct MultiplayerClientPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    On,
    Off,
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
        let com: ComClient = {
            let mut ret = None;
            for i in 0..10 {
                let mut addr = base_addr.to_string();
                addr.push_str(&(base_local_port + i).to_string());
                if let Ok(com) = ComClient::connect(&addr, remote_addr) {
                    ret = Some(com);
                    break;
                }
            }
            ret.unwrap()
        };

        let value = ClientMessage {
            command: Command::MoveDirection(Vec2 { x: 0.0, y: 10.0 }),
        };
        com.send(&value);
        app.add_state(State::Off);

        app.insert_resource(com);
        app.insert_resource(MessagesToRead::default());
        app.insert_resource(MessagesToSend::default());
        app.add_system_set(
            SystemSet::on_update(State::On)
                .with_system(receive_messages.system())
                .with_system(send_messages.system()),
        );
    }
}

fn receive_messages(
    mut com_to_read: ResMut<ComClient>,
    mut messages_to_read: ResMut<MessagesToRead>,
) {
    while let Some(msg) = match com_to_read.receive() {
        Ok(msg) => Some(msg),
        Err(err) => {
            //dbg!("error: {}", err);
            return;
        }
    } {
        messages_to_read.messages.push_back(msg);
    }
}

fn send_messages(mut com_to_send: ResMut<ComClient>, mut messages_to_send: ResMut<MessagesToSend>) {
    for msg in messages_to_send.messages.iter() {
        com_to_send.send(msg);
    }
    messages_to_send.messages.clear();
}
