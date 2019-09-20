extern crate sdl2;

fn main() {
    // Zmienne

    // inicjalizacja sdl2
    let sdl = sdl2::init().unwrap();
    /* inicjalizacja podsystemu video
    Wziąłem to z tutorialu*/
    let video_subsystem = sdl.video().unwrap();
    /* Stworzenie okna gry, też z tutorialu */
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable() /* Żeby można była zmieniać wielkość */
        .build()
        .unwrap();

    let mut renderer = window
        .into_canvas()
        .present_vsync() /* Max klatek jest tyle co monitor ma */
        .build()
        .unwrap();

    /* Czytanie Event'ów*/
    let mut event_pump = sdl.event_pump().unwrap();

    /* Tworzy punkt do wyświetlenia linii */
    let mut point = sdl2::rect::Point::new(20,20);

    /* Głowna pętla, adnotacja 'main by móc ją potem zamknąć */
    'main: loop {
            // Reakcja na event'y
        for event in event_pump.poll_iter() {
            use sdl2::event::Event; /* By mieć event w scope */
            use sdl2::keyboard::Keycode; /* By mieć kody klawiszy w scope */
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    point.x -= 10
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    point.x += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    point.y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    point.y += 10;
                },
                _ => {},
            }
        }
        renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        /* Wyświetla linię*/
        renderer.draw_line(sdl2::rect::Point::new(20,20), point).unwrap(); /* unwrap jest bo chce by użyć wyniku
        ( OK() lub Err() )*/
        renderer.present();
        std::thread::sleep(std::time::Duration::from_millis(50))
    }
}
