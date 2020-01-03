use specs::{Component, VecStorage};

// Resource
#[derive(Default)]
pub struct PInput {
   pub up: bool,
   pub down: bool,
   pub right: bool,
   pub left: bool,
}

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

#[derive(Component)]
#[storage(VecStorage)]
pub struct Sprite {
   pub img: String,
}
