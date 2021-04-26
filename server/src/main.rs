use std::{thread, time};

use network::{Communicator, MessageHandler};
use shared::{ClientMessage, ServerMessage};

pub mod network;

pub struct ClientMessageHandler {}

impl MessageHandler<ClientMessage> for ClientMessageHandler {
    fn handle(message: ClientMessage) {
        println!("received: {:#?}", message);
    }
}
fn main() -> Result<(), std::io::Error> {
    let handler = ClientMessageHandler {};
    let mut communicator = Communicator::connect("127.0.0.1:8080");
    loop {
        let (client, message) = communicator.receive()?;
        println!("received: {:#?}", message);
        let world = shared::World {
            entities: vec![],
            bullets: vec![],
        };
        communicator.send(&client, ServerMessage { world });
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
    Ok(())
}
