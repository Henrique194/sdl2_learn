use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

    let windows_builder = sdl_ctx.video().unwrap()
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

fn load_media() -> Option<HashMap<i32, Surface<'static>>> {
    let mut surfaces = HashMap::new();
    surfaces.insert(0, match load_surface("imgs/key_presses/press.bmp") {
        None => {
            println!("Failed to load default image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Keycode::Up as i32, match load_surface("imgs/key_presses/up.bmp") {
        None => {
            println!("Failed to load up image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Keycode::Down as i32, match load_surface("imgs/key_presses/down.bmp") {
        None => {
            println!("Failed to load down image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Keycode::Left as i32, match load_surface("imgs/key_presses/left.bmp") {
        None => {
            println!("Failed to load left image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Keycode::Right as i32, match load_surface("imgs/key_presses/right.bmp") {
        None => {
            println!("Failed to load right image!");
            return None;
        },
        Some(surface) => surface
    });

    Some(surfaces)
}

fn load_surface(path: &str) -> Option<Surface> {
    match Surface::load_bmp(path) {
        Err(error) => {
            println!("Unable to load image {}! SDL Error: {}", path, error);
            None
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

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let key_press_surfaces = match load_media() {
        None => {
            println!("Failed to load media!");
            return;
        },
        Some(surfaces) => surfaces
    };

    let mut current_screen = &key_press_surfaces[&0];

    'running: loop {
        let event = match event_pump.poll_event() {
            Some(event) => event,
            None => continue
        };

        if let Event::Quit {..} = event {
            break 'running;
        }

        if let Event::KeyDown {keycode, ..} = event {
            let keycode = keycode.unwrap();
            match keycode {
                Keycode::Up | Keycode::Down
                | Keycode::Left | Keycode::Right => {
                    current_screen = &key_press_surfaces[&(keycode as i32)]
                },
                _ => current_screen = &key_press_surfaces[&0]
            };
        }

        let mut screen_surface = window.surface(&event_pump).unwrap();
        current_screen.blit(None, &mut screen_surface, None).expect("");
        screen_surface.update_window().unwrap();
    }
}