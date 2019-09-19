extern crate sdl2;

fn main() {
    // inicjalizacja sdl2
    let sdl = sdl2::init().unwrap();
    /* inicjalizacja podsystemu video
    Wziąłem to z tutorialu*/
    let video_subsystem = sdl.video().unwrap();
    /* Stworzenie okna gry, też z tutorialu */
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();

    /* Czytanie Event'ów*/
    let mut event_pump = sdl.event_pump().unwrap();

    /* Głowna pętla, adnotacja 'main by móc ją potem zamknąć */
    'main: loop {
            // Reakcja na event'y
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        

    }
}
