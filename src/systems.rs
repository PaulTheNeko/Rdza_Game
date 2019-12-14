use crate::components::*;
use specs::prelude::*;
use cgmath;

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

pub struct PInputAddVel;

impl<'a> System<'a> for PInputAddVel {
    type SystemData = (Read<'a, PInput>,
                        WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        use cgmath::Vector2;
        let (pinput, mut vel) = data;
        let mut AddVel = Vector2::new(0.0, 0.0);

        // Słabe
        // Sumuje kierunki
        if pinput.up {AddVel = Vector2::new(0.0, -0.1)};
        if pinput.down {AddVel = Vector2::new(0.0, 0.1)};
        if pinput.left {AddVel = Vector2::new(-0.1, 0.0)};
        if pinput.right {AddVel = Vector2::new(0.1, 0.0)};

        for mut vel in (&mut vel).join() {
            vel.x = vel.x + AddVel.x;
            vel.y = vel.y + AddVel.y;
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
