mod components;
mod systems;
use crate::components::*;
use crate::systems::*;

use std::collections::HashMap;

use cgmath;
use ggez;
use specs::prelude::*;

fn main() {
   let PInput = PInput {
      up: false,
      down: false,
      right: false,
      left: false,
   };

   // Świat dla specs, zawiera byty
   let mut world = World::new();
   world.insert(PInput);
   world.register::<Position>();
   world.register::<Velocity>();
   world.register::<Sprite>();

   // Taki byt dla testów
   let _example = world
      .create_entity()
      .with(Position { x: 4.0, y: 7.0 })
      .with(Velocity { x: 1.0, y: 0.0 })
      .with(Sprite { img: "example".to_string() })
      .build();

   // Dispatcher, takie coś do włączania systemów
   // Aktualnie używany tylko do logiki gry
   let dispatcher = DispatcherBuilder::new()
      .with(UpdatePos, "update_pos", &[])
      .with(PInputAddVel, "playerinput_add_velocity", &[])
      .build();

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
      dispatcher,
   };

   // Pętla
   ggez::event::run(ctx, event_loop, state).unwrap();
}

// Stan świata dla ggez
struct State {
   world: World,
   textures: HashMap<String, ggez::graphics::Image>,
   dispatcher: Dispatcher<'static, 'static>,
}

// Kod do reagowania na event'y od ggez
impl ggez::event::EventHandler for State {
   // Kod do logiki gry
   // Nie trzeba zmieniać
   fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
      self.dispatcher.dispatch(&self.world); // Włącza systemy
      self.world.maintain(); // Potrzebne
      Ok(())
   }

   // Renderowanie i wyświetlanie
   // Może być do tego później potrzebny drugi dispatcher
   fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
      use ggez::graphics;
      PrintSystem.run_now(&self.world); // Wyświetla w terminalu pozycje obiektów
      let clr: graphics::Color = (0.0, 0.0, 0.0).into(); // czarny
      graphics::clear(ctx, clr); // Czyści ekran

      let pos = self.world.read_storage::<Position>(); // Pozycja
      let sprite = self.world.read_storage::<Sprite>(); // Id tekstury
      for (p, img) in (&pos, &sprite).join() {
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
      std::thread::yield_now(); // Daje systemowi odetchnąć
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
      let pinput = &mut self.world.write_resource::<PInput>();

      match keycode {
         KeyCode::W => pinput.up = true,
         KeyCode::S => pinput.down = true,
         KeyCode::A => pinput.left = true,
         KeyCode::D => pinput.right = true,
         _ => (),
      }
   }

   fn key_up_event(
      &mut self,
      ctx: &mut ggez::Context,
      keycode: ggez::event::KeyCode,
      _keymods: ggez::event::KeyMods,
   ) {
      use ggez::event::KeyCode;
      let pinput = &mut self.world.write_resource::<PInput>();

      match keycode {
         KeyCode::W => pinput.up = false,
         KeyCode::S => pinput.down = false,
         KeyCode::A => pinput.left = false,
         KeyCode::D => pinput.right = false,
         _ => (),
      }
   }
}
