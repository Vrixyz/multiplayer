pub mod network;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Command {
    MoveDirection(Vec2),
    Shoot(Vec2),
    Aim(Vec2),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ClientMessage {
    pub command: Command,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Entity {
    pub position: Vec2,
    pub size: f32,
    pub id: usize,
    pub team: usize,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct World {
    pub entities: Vec<Entity>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ServerMessage {
    pub world: World,
}

#[derive(Debug)]
pub struct Id(pub usize);
