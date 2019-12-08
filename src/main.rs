mod systems;
mod components;
use crate::systems::*;
use crate::components::*;

use ggez;
use specs::prelude::*;
use cgmath;

fn main() {
   // Świat dla specs, zawiera byty
   let mut world = World::new();
   world.register::<Position>();
   world.register::<Velocity>();

   // Taki byt dla testów
   let ball = world.create_entity()
      .with(Position { x: 4.0, y: 7.0 })
      .with(Velocity { x: 1.0, y: 0.0 })
      .build();

   // Dispatcher, takie coś do włączania systemów
   // Aktualnie używany tylko do logiki gry
   let mut dispatcher = DispatcherBuilder::new()
      .with(UpdatePos, "update_pos", &[])
      .build();

   // Stan świata dla ggez
   let state = &mut State { 
      world: world,
      dispatcher: dispatcher,
   };

   // Ustawienia okna.
   // Wszystko podstawowe, tylko żeby rozmiar można było zmieniać
   let c = ggez::conf::Conf {
      window_mode: ggez::conf::WindowMode::default()
                                    .resizable(true),
      window_setup: ggez::conf::WindowSetup::default(),
      backend: ggez::conf::Backend::default(),
      modules: ggez::conf::ModuleConf::default(),
   };

   // Context dla ggez, okno i eventloop
   let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("hello_ggez", "Paweł_Nowiński")
    .conf(c)
    .build()
    .unwrap();
   
   ggez::event::run(ctx, event_loop, state).unwrap();
}

// Stan świata dla ggez
struct State {
   world: World,
   dispatcher: Dispatcher<'static, 'static>,
}

// Kod do reagowania na event'y od ggez
impl ggez::event::EventHandler for State {

   // Kod do logiki gry
   // Nie trzeba zmieniać
   fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
      self.dispatcher.dispatch(&self.world); // Włącza systemy
      self.world.maintain(); // Potrzebne
      Ok(())
  }

   // Renderowanie i wyświetlanie
   // Może być do tego później potrzebny drugi dispatcher
   fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
      PrintSystem.run_now(&self.world); // Wyświetla w terminalu pozycje obiektów
      Ok(()) // Context dla ggez, okno i eventloop
  }
}