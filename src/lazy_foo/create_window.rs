use sdl2::event::Event;
use sdl2::pixels::Color;


pub fn run() {
    const SCREEN_WIDTH: u32 = 640;
    const SCREEN_HEIGHT: u32 = 480;

    let sdl_ctx = match sdl2::init() {
        Err(error) => {
            println!("SDL could not initialize! SDL_Error: {error}");
            return;
        }
        Ok(sdl) => sdl
    };

    let video_subsystem = match sdl_ctx.video() {
        Err(error) => {
            println!("Window could not be created! SDL_Error: {error}");
            return;
        }
        Ok(video_subsystem) => video_subsystem
    };

    let windows_builder = video_subsystem
        .window("SDL Tutorial", SCREEN_WIDTH, SCREEN_HEIGHT);

    let window = match windows_builder.build() {
        Err(error) => {
            println!("Window could not be created! SDL_Error: {error}");
            return;
        }
        Ok(window) => window
    };

    let mut event_pump = match sdl_ctx.event_pump() {
        Err(error) => {
            println!("Window could not be created! SDL_Error: {error}");
            return;
        }
        Ok(event_pump) => event_pump
    };

    let mut screen_surface = match window.surface(&event_pump) {
        Err(_error) => return,
        Ok(screen_surface) => screen_surface
    };

    match screen_surface.fill_rect(None, Color::RGB(0xFF, 0xFF, 0xFF)) {
        Err(_error) => return,
        _ => ()
    }

    match screen_surface.update_window() {
        Err(_error) => return,
        _ => ()
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => ()
            }
        }
    }
}
