// TODO: hide rkyv from lib use
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub enum Command {
    Move(Vec2),
    Shoot(Vec2),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct ClientMessage {
    pub command: Command,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Entity {
    pub position: Vec2,
    pub velocity: Vec2,
    pub id: usize,
    pub team: usize,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct World {
    pub entities: Vec<Entity>,
    pub bullets: Vec<Entity>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct ServerMessage {
    pub world: World,
}