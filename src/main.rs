
use ggez;
use specs::prelude::*;
use specs::{Component, VecStorage};

fn main() {
   let mut world = World::new();
   world.register::<Position>();
   world.register::<Velocity>();
   let ball = world.create_entity()
      .with(Position { x: 4.0, y: 7.0 })
      .with(Velocity { x: 1.0, y: 0.0 })
      .build();

   let mut dispatcher = DispatcherBuilder::new()
      .with(UpdatePos, "update_pos", &[])
      .build();


   let state = &mut State { 
      world: world,
      dispatcher: dispatcher,
   };

   let c = ggez::conf::Conf::new();
   let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("hello_ggez", "awesome_person")
    .conf(c)
    .build()
    .unwrap();
   
   ggez::event::run(ctx, event_loop, state).unwrap();
}

struct State {
   world: World,
   dispatcher: Dispatcher<'static, 'static>,
}

impl ggez::event::EventHandler for State{
   fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
      self.dispatcher.dispatch(&self.world);
      self.world.maintain();
      Ok(())
  }
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
      PrintSystem.run_now(&self.world);
      Ok(())
  }
}


#[derive(Component)]
#[storage(VecStorage)]
struct Position {
   x: f32,
   y: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
struct Velocity {
   x: f32,
   y: f32,
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
   type SystemData = (ReadStorage<'a, Velocity>,
                      WriteStorage<'a, Position>);
    
   fn run(&mut self, (vel, mut pos): Self::SystemData) {
      for (vel,pos) in (&vel, &mut pos).join() {
         pos.x += vel.x;
         pos.y += vel.y;
      }
   }
}

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
   type SystemData = (ReadStorage<'a, Position>);

   fn run(&mut self, pos: Self::SystemData) {
      for pos in (&pos).join() {
         println!("x:{} y:{}", pos.x, pos.y)
      }
   }
}