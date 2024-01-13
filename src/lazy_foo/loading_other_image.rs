use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::{Sdl};
use sdl2::image::{InitFlag, LoadSurface, Sdl2ImageContext};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowSurfaceRef};


const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn init() -> Result<(Sdl, Sdl2ImageContext, Window), String> {
    let sdl = match sdl2::init() {
        Err(error) => {
            let str = format!("SDL could not initialize! SDL_Error: {}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(sdl) =>  sdl
    };

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

    let sdl_img = match sdl2::image::init(InitFlag::PNG) {
        Err(error) => {
            let str = format!("SDL_image could not initialize! SDL_image Error: {}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(sdl2_img) => sdl2_img
    };

    Ok((sdl, sdl_img, window))
}

fn load_media<'a>(screen_surface: &WindowSurfaceRef) -> Result<Surface<'a>, String> {
    let stretched_surface =
        load_surface("imgs/loading_other_image/loaded.png", screen_surface);

    if stretched_surface.is_err() {
        let str = String::from("Failed to load PNG image!");
        println!("{}", str);
        return Err(str);
    }

    stretched_surface
}

fn load_surface<'a>(path: &str, screen_surface: &WindowSurfaceRef) -> Result<Surface<'a>, String> {
    let loaded_surface = match Surface::from_file(path) {
        Err(error) => {
            let str = format!("Unable to load image {}! SDL Error: {}", path, error);
            println!("{}", str);
            return Err(str);
        },
        Ok(surface) => surface
    };

    match loaded_surface.convert(&screen_surface.pixel_format()) {
        Err(error) => {
            let str = format!("Unable to optimize image {}! SDL Error: {}", path, error);
            println!("{}", str);
            Err(str)
        },
        Ok(surface) => Ok(surface)
    }
}

pub fn run() {
    // Necessary to keep SDL2 Image Context alive
    let (sdl, _sdl_img, window) = match init() {
        Err(_) => {
            println!("Failed to initialize!");
            return;
        }
        Ok(tuple) => tuple
    };

    let mut event_pump = sdl.event_pump().unwrap();
    let mut screen_surface = window.surface(&event_pump).unwrap();

    let stretched_surface = match load_media(&screen_surface) {
        Err(_) => {
            println!("Failed to load media!");
            return;
        },
        Ok(surfaces) => surfaces
    };

    let stretch_rec = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    'running: loop {
        let event = match event_pump.poll_event() {
            Some(event) => event,
            None => continue
        };

        if let Event::Quit {..} = event {
            break 'running;
        }

        screen_surface = window.surface(&event_pump).unwrap();
        stretched_surface.blit_scaled(None, &mut screen_surface, stretch_rec)
            .expect("Should blit correctly");
        screen_surface.update_window().expect("Should update correctly");
    }
}