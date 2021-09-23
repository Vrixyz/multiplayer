pub use bevy::prelude::*;
pub use bevy_prototype_debug_lines::DebugLines;

pub mod attack;
pub mod movement;
pub mod physics;

pub struct Unit {
    pub client_id: usize,
}

#[derive(Default, Debug)]
pub struct IdProvider {
    next_free_id: usize,
}

impl IdProvider {
    pub fn new_id(&mut self) -> usize {
        let id = self.next_free_id;
        self.next_free_id = self
            .next_free_id
            .checked_add(1)
            .expect("IdProvider created too many ids and cannot handle it. Call the dev to work on that issue.");
        id
    }
}
