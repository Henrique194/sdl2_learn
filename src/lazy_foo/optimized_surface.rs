use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::Sdl;
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowSurfaceRef};


const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn init() -> Option<(Sdl, Window)> {
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

fn load_media<'a>(screen_surface: &WindowSurfaceRef) -> Option<Surface<'a>> {
    let stretched_surface =
        load_surface("imgs/optimized_surface/stretch.bmp", screen_surface);

    if stretched_surface.is_none() {
        println!("Failed to load stretching image!");
    };

    stretched_surface
}

fn load_surface<'a>(path: &str, screen_surface: &WindowSurfaceRef) -> Option<Surface<'a>> {
    let loaded_surface = match Surface::load_bmp(path) {
        Err(error) => {
            println!("Unable to load image {}! SDL Error: {}", path, error);
            return None;
        },
        Ok(surface) => surface
    };

    match loaded_surface.convert(&screen_surface.pixel_format()) {
        Err(error) => {
            println!("Unable to optimize image {}! SDL Error: {}", path, error);
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
    let mut screen_surface = window.surface(&event_pump).unwrap();

    let stretched_surface = match load_media(&screen_surface) {
        None => {
            println!("Failed to load media!");
            return;
        },
        Some(surfaces) => surfaces
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