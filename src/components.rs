//use specs::{Component, VecStorage};
//use legion::prelude::*;

// Resource
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlayerInput {
   pub up: bool,
   pub down: bool,
   pub left: bool,
   pub right: bool,
}

// Resource
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CamPos(pub cgmath::Point2<f32>);

// Tag
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position(pub cgmath::Point2<f32>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity (pub cgmath::Vector2<f32>);

#[derive(Clone, Debug, PartialEq)]
pub struct Sprite {
   pub img: String,
}
