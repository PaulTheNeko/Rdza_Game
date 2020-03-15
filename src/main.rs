mod components;
use crate::components::*;

use std::collections::HashMap;

use cgmath;
use ggez;
// use specs::prelude::*;
use legion::prelude::*;

fn main() {
   let playerinput = PlayerInput {
      up: false,
      down: false,
      right: false,
      left: false,
   };

   // Świat dla Legion, zawiera byty
   let universe = Universe::new();
   let mut world = universe.create_world();

   let mut resources = Resources::default();
   resources.insert(playerinput);

   world.resources = resources;

   // Byt dla testów
   world.insert(
      (),
      vec![
         (Position{x: 0.0, y:0.0}, Velocity{ x: 0.1, y:0.2}, Sprite{ img:"example".to_string()})
         ]
   );

   // -- Systemy --
   let mut systems = Vec::new();
   systems.push( SystemBuilder::<()>::new("update_positions")
         .with_query(<(Write<Position>, Read<Velocity>)>::query())
         .build(|_, mut world, _ /*resources*/, queries| {
            for (mut pos, vel) in queries.iter(&mut *world) {
               pos.x += vel.x;
               pos.y += vel.y;
            }
         })
   );


   systems.push(SystemBuilder::<()>::new("add_velocity_from_playerinput")
         .read_resource::<PlayerInput>()
         .with_query(<(Write<Velocity>)>::query())
         .build(|_, mut world, res1, queries| {
            for (mut vel) in queries.iter(&mut *world) {
               if res1.up {vel.y = vel.y - 0.1};
               if res1.down {vel.y = vel.y + 0.1};
               if res1.left {vel.x = vel.x - 0.1};
               if res1.right {vel.x = vel.x + 0.1};
            }
         })
   );

   // Schedule
   //
   // Nie mogę dynamicznie zbudować bo jest move
   let mut schedule = Schedule::builder();
   for i in systems {
      schedule = schedule.add_system(i);
   }
   let schedule = schedule.build();

   // Ustawienia okna.
   // Wszystko podstawowe, tylko żeby rozmiar można było zmieniać
   let c = ggez::conf::Conf {
      window_mode: ggez::conf::WindowMode::default().resizable(true),
      window_setup: ggez::conf::WindowSetup::default(),
      backend: ggez::conf::Backend::default(),
      modules: ggez::conf::ModuleConf::default(),
   };

   // Context dla ggez, okno i eventloop
   let (ref mut ctx, ref mut event_loop) =
      ggez::ContextBuilder::new("hello_ggez", "Paweł_Nowiński")
         .conf(c)
         .build()
         .unwrap();

   // Tekstury
   // Aktualnie tylko jedna
   let mut textures: HashMap<String, ggez::graphics::Image> = HashMap::new();
   let path = std::path::Path::new("/example.png");
   let mut img = ggez::graphics::Image::new(ctx, path).unwrap();
   img.set_filter(ggez::graphics::FilterMode::Nearest);
   textures.insert("example".to_string(), img);

   // Stan świata dla ggez
   let state = &mut State {
      world,
      textures,
      schedule,
   };

   // Pętla
   ggez::event::run(ctx, event_loop, state).unwrap();
}

// Stan świata dla ggez
struct State {
   world: World,
   textures: HashMap<String, ggez::graphics::Image>,
   // dispatcher: Dispatcher<'static, 'static>,
   schedule: Schedule,
}

// Kod do reagowania na event'y od ggez
impl ggez::event::EventHandler for State {
   // Kod do logiki gry
   // Nie trzeba zmieniać
   fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
      self.schedule.execute(&mut self.world); // Włącza systemy
      Ok(())
   }

   // Renderowanie i wyświetlanie
   // Może być do tego później potrzebny drugi dispatcher
   fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
      use ggez::graphics;
      let clr: graphics::Color = (0.0, 0.0, 0.0).into(); // czarny
      graphics::clear(ctx, clr); // Czyści ekran

      let query = <(Read<Position>, Read<Sprite>)>::query();
      for (p, img) in query.iter(&mut self.world) {
         let img = self.textures.get(&img.img).unwrap(); // Wyciąga teksturę z HashMap

         graphics::draw(
            ctx,
            img,
            graphics::DrawParam::new()
               .dest(cgmath::Point2::new(p.x, p.y))
               .scale(cgmath::Vector2::new(10.0, 10.0)),
         )?;
      }

      graphics::present(ctx)?;
      std::thread::yield_now(); // Daje systemowi (OS) odetchnąć
      Ok(()) // Jakby był błąd to funkcja by spanikowała
   }

   fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) -> () {
      use ggez::graphics;
      let coord = graphics::Rect::new(0.0, 0.0, width, height);
      let _ = graphics::set_screen_coordinates(ctx, coord);
      ()
   }

   fn key_down_event(
      &mut self,
      _ctx: &mut ggez::Context,
      keycode: ggez::event::KeyCode,
      _keymods: ggez::event::KeyMods,
      _repeat: bool,
   ) {
      use ggez::event::KeyCode;
      let mut playerinput = self.world.resources.get_mut::<PlayerInput>().unwrap();

      match keycode {
         KeyCode::W => playerinput.up = true,
         KeyCode::S => playerinput.down = true,
         KeyCode::A => playerinput.left = true,
         KeyCode::D => playerinput.right = true,
         _ => (),
      }
      
   }

   fn key_up_event(
      &mut self,
      _ctx: &mut ggez::Context,
      keycode: ggez::event::KeyCode,
      _keymods: ggez::event::KeyMods,
   ) {
      use ggez::event::KeyCode;
      let mut playerinput = &mut self.world.resources.get_mut::<PlayerInput>().unwrap();

      match keycode {
         KeyCode::W => playerinput.up = false,
         KeyCode::S => playerinput.down = false,
         KeyCode::A => playerinput.left = false,
         KeyCode::D => playerinput.right = false,
         _ => (),
      }
   }
}