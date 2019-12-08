use specs::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
   pub x: f32,
   pub y: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
   pub x: f32,
   pub y: f32,
}
