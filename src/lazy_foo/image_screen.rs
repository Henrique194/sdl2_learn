use sdl2::event::Event;
use sdl2::Sdl;
use sdl2::surface::Surface;
use sdl2::video::Window;


fn init() -> Option<(Sdl, Window)> {
    const SCREEN_WIDTH: u32 = 640;
    const SCREEN_HEIGHT: u32 = 480;

    let sdl_ctx = match sdl2::init() {
        Err(error) => {
            println!("SDL could not initialize! SDL_Error: {}", error);
            return None;
        },
        Ok(sdl) =>  sdl
    };

    let video_subsystem = match sdl_ctx.video() {
        Err(error) => {
            println!("Window could not be created! SDL_Error: %{}", error);
            return None;
        },
        Ok(window) =>  window
    };

    let windows_builder = video_subsystem
        .window("SDL Tutorial", SCREEN_WIDTH, SCREEN_HEIGHT);

    let window = match windows_builder.build() {
        Err(error) => {
            println!("Window could not be created! SDL_Error: %{}", error);
            return None;
        },
        Ok(window) => window
    };

    Some((sdl_ctx, window))
}

fn load_image() -> Option<Surface<'static>> {
    let path = "imgs/image_screen/hello_world.bmp";

    match Surface::load_bmp(path) {
        Err(error) => {
            println!("Unable to load image {}! SDL Error: {}", path, error);
            return None
        },
        Ok(surface) => Some(surface)
    }
}

pub fn run() {
    let (sdl_ctx, window) = match init() {
        None => {
            println!("Failed to initialize!");
            return
        },
        Some(tuple) => tuple
    };

    let mut event_pump =  match sdl_ctx.event_pump() {
        Err(error) => {
            println!("Event pump could not be created! SDL_Error: %{}", error);
            return;
        },
        Ok(event) => event
    };

    let mut screen_surface = match window.surface(&event_pump) {
        Err(_) =>  return,
        Ok(surface) => surface
    };

    let hello_world_surface = match load_image() {
        None => {
            println!("Failed to load media!");
            return;
        },
        Some(surface) => surface
    };

    match hello_world_surface.blit(None, &mut screen_surface, None) {
        Err(_) => return,
        _ => ()
    };

    match screen_surface.update_window() {
        Err(_) => return,
        _ => ()
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => ()
            }
        }
    }
}
