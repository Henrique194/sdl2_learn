use std::collections::HashMap;
use sdl2::controller::Button;

use sdl2::event::Event;
use sdl2::hint::set;
use sdl2::rwops::RWops;
use sdl2::Sdl;
use sdl2::surface::Surface;
use sdl2::video::Window;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn init() -> Result<(Sdl, Window), String> {
    let sdl = match sdl2::init() {
        Err(error) => {
            let str = format!("SDL could not initialize! SDL_Error: {}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(sdl) =>  sdl
    };

    if !set("SDL_RENDER_SCALE_QUALITY", "1") {
        println!("Warning: Linear texture filtering not enabled!")
    }

    let windows_builder = sdl.video().unwrap()
        .window("SDL Tutorial", SCREEN_WIDTH, SCREEN_HEIGHT);

    let window = match windows_builder.build() {
        Err(error) => {
            let str = format!("Window could not be created! SDL_Error: %{}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(window) => window
    };

    Ok((sdl, window))
}

fn load_media() -> Option<HashMap<i32, Surface<'static>>> {
    let mut surfaces = HashMap::new();
    surfaces.insert(-2, match load_surface("imgs/gamepads_and_joysticks/press.bmp") {
        None => {
            println!("Failed to load default image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Button::A as i32, match load_surface("imgs/gamepads_and_joysticks/A.bmp") {
        None => {
            println!("Failed to load up image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Button::B as i32, match load_surface("imgs/gamepads_and_joysticks/B.bmp") {
        None => {
            println!("Failed to load down image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Button::X as i32, match load_surface("imgs/gamepads_and_joysticks/X.bmp") {
        None => {
            println!("Failed to load left image!");
            return None;
        },
        Some(surface) => surface
    });
    surfaces.insert(Button::Y as i32, match load_surface("imgs/gamepads_and_joysticks/Y.bmp") {
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
    let (sdl, window) = match init() {
        Err(_) => {
            println!("Failed to initialize!");
            return;
        }
        Ok(tuple) => tuple
    };

    let key_press_surfaces = match load_media() {
        None => {
            println!("Failed to load media!");
            return;
        },
        Some(surfaces) => surfaces
    };

    let mut current_screen = key_press_surfaces.get(&-2).unwrap();

    let joystick = match sdl.game_controller() {
        Err(error) => {
            let str = format!("SDL could not initialize! SDL Error: {}", error);
            println!("{}", str);
            return;
        },
        Ok(joystick) => joystick
    };

    let rw = RWops::from_file("controller_mapping.txt", "rb").unwrap();
    joystick.load_mappings_from_rw(rw).unwrap();

    let _game_controller = joystick.open(0).unwrap();

    if joystick.num_joysticks().unwrap() < 1 {
        println!("Warning: No joysticks connected!");
        return;
    }

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        let event = match event_pump.poll_event() {
            Some(event) => event,
            None => continue
        };

        if let Event::Quit {..} = event {
            break 'running;
        }

        match event {
            Event::ControllerButtonDown { button, .. } => {
                match button {
                    Button::A
                    | Button::B
                    | Button::X
                    | Button::Y  => {
                        current_screen = key_press_surfaces.get(&(button as i32)).unwrap();
                    },
                    _ => current_screen = key_press_surfaces.get(&-2).unwrap()
                };
            },
            Event::KeyDown {..} => current_screen = key_press_surfaces.get(&-2).unwrap(),
            _ => ()
        }

        let mut screen_surface = window.surface(&event_pump).unwrap();
        current_screen.blit(None, &mut screen_surface, None).expect("");
        screen_surface.update_window().unwrap();
    }
}