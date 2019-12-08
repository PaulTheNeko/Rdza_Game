use crate::components::*;
use specs::prelude::*;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (ReadStorage<'a, Position>);

    fn run(&mut self, pos: Self::SystemData) {
        for pos in (&pos).join() {
            println!("x:{} y:{}", pos.x, pos.y)
        }
    }
}
