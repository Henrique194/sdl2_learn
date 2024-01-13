use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::hint::set;
use sdl2::image::{InitFlag, LoadSurface, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::render::{CanvasBuilder, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::{WindowContext};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn init() -> Result<(Sdl, Sdl2ImageContext, WindowCanvas), String> {
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

    let mut renderer = match CanvasBuilder::new(window).accelerated().build() {
        Err(error) => {
            let str = format!("Renderer could not be created! SDL Error: {}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(canvas) => canvas
    };
    renderer.set_draw_color(Color::WHITE);

    let sdl2_img = match sdl2::image::init(InitFlag::PNG) {
        Err(error) => {
            let str = format!("SDL_image could not initialize! SDL_image Error: {}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(sdl2_img) => sdl2_img
    };

    Ok((sdl, sdl2_img, renderer))
}

fn load_media(texture_creator: &TextureCreator<WindowContext>) -> Result<Texture, String> {
    let texture =
        load_texture("imgs/texture_loading/texture.png", texture_creator);

    if texture.is_err() {
        let str = String::from("Failed to load PNG image!");
        println!("{}", str);
        return Err(str);

    }

    texture
}

fn load_texture<'a>(
    path: &str,
    texture_creator: &'a TextureCreator<WindowContext>
) -> Result<Texture<'a>, String> {
    let loaded_surface = match Surface::from_file(path) {
        Err(error) => {
            let str = format!("Unable to load image {}! SDL Error: {}", path, error);
            println!("{}", str);
            return Err(str);
        },
        Ok(surface) => surface
    };

    match texture_creator.create_texture_from_surface(loaded_surface) {
        Err(error) => {
            let str = format!("Unable to create texture from {}! SDL Error: {}", path, error);
            println!("{}", str);
            Err(str)
        },
        Ok(texture) => Ok(texture)
    }
}

pub fn run() {
    // Necessary to keep SDL2 Image Context alive
    let (sdl, _sdl_img, mut renderer) = match init() {
        Err(_) => {
            println!("Failed to initialize!");
            return;
        }
        Ok(tuple) => tuple
    };

    let texture_creator = renderer.texture_creator();

    let texture = match load_media(&texture_creator) {
        Err(_) => {
            println!("Failed to load media!");
            return;
        },
        Ok(texture) => texture
    };

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        let event = match event_pump.poll_event() {
            Some(event) => event,
            None => continue
        };

        if let Event::Quit {..} = event {
            break 'running;
        }

        renderer.clear();
        renderer.copy(&texture, None, None).expect("Should not fail!");
        renderer.present();
    }
}