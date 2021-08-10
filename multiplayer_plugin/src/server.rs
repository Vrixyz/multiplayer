use std::collections::VecDeque;

use bevy::prelude::*;
use shared::{
    network::com_server::{Client, ComServer},
    ClientMessage, ServerMessage,
};

pub struct MultiplayerServerPlugin;

#[derive(Default)]
pub struct MessagesToSend {
    messages: VecDeque<(Client, ServerMessage)>,
}
impl MessagesToSend {
    pub fn push(&mut self, message: (Client, ServerMessage)) {
        self.messages.push_back(message);
    }
}

#[derive(Default)]
pub struct MessagesToRead {
    messages: VecDeque<(Client, ClientMessage)>,
}
impl MessagesToRead {
    pub fn pop(&mut self) -> Option<(Client, ClientMessage)> {
        self.messages.pop_front()
    }
}
impl Plugin for MultiplayerServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let com = ComServer::bind("127.0.0.1:8083");

        app.insert_resource(com);
        app.insert_resource(MessagesToRead::default());
        app.insert_resource(MessagesToSend::default());
        app.add_system(receive_messages.system());
        app.add_system(send_messages.system());
    }
}

fn receive_messages(
    mut com_to_read: ResMut<ComServer>,
    mut messages_to_read: ResMut<MessagesToRead>,
) {
    while let Ok(msg) = com_to_read.receive() {
        println!("{:#?}", msg);

        messages_to_read.messages.push_back(msg);
    }
}

fn send_messages(mut com_to_send: ResMut<ComServer>, mut messages_to_send: ResMut<MessagesToSend>) {
    for msg in messages_to_send.messages.iter() {
        com_to_send.send(&msg.0, &msg.1);
    }
    messages_to_send.messages.clear();
}
