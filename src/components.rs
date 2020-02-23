//use specs::{Component, VecStorage};

// Resource
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlayerInput {
   pub up: bool,
   pub down: bool,
   pub right: bool,
   pub left: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
   pub x: f32,
   pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
   pub x: f32,
   pub y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sprite {
   pub img: String,
}
