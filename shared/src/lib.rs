pub mod network;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub enum Command {
    MoveDirection(Vec2),
    Shoot(Vec2),
    Aim(Vec2),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct ClientMessage {
    pub command: Command,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Entity {
    pub position: Vec2,
    pub velocity: Vec2,
    pub id: usize,
    pub team: usize,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct World {
    pub entities: Vec<Entity>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ServerMessage {
    pub world: World,
}

#[derive(Debug)]
pub struct Id(pub usize);
