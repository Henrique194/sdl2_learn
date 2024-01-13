use sdl2::event::Event;
use sdl2::hint::set;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{CanvasBuilder, WindowCanvas};
use sdl2::Sdl;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn init() -> Result<(Sdl, WindowCanvas), String> {
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

    let renderer = match CanvasBuilder::new(window).accelerated().build() {
        Err(error) => {
            let str = format!("Renderer could not be created! SDL Error: {}", error);
            println!("{}", str);
            return Err(str);
        },
        Ok(canvas) => canvas
    };

    Ok((sdl, renderer))
}

pub fn run() {
    // Necessary to keep SDL2 Image Context alive
    let (sdl, mut renderer) = match init() {
        Err(_) => {
            println!("Failed to initialize!");
            return;
        }
        Ok(tuple) => tuple
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

        renderer.set_draw_color(Color::WHITE);
        renderer.clear();

        let fill_rect = Rect::new(
            (SCREEN_WIDTH / 4) as i32,
            (SCREEN_HEIGHT / 4) as i32,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT / 2
        );
        renderer.set_draw_color(Color::RED);
        renderer.fill_rect(fill_rect).unwrap();

        let outline_rect = Rect::new(
            (SCREEN_WIDTH / 6) as i32,
            (SCREEN_HEIGHT / 6) as i32,
            SCREEN_WIDTH * 2 / 3,
            SCREEN_HEIGHT * 2 / 3
        );
        renderer.set_draw_color(Color::GREEN);
        renderer.draw_rect(outline_rect).unwrap();

        renderer.set_draw_color(Color::BLUE);
        renderer.draw_line((0, (SCREEN_HEIGHT / 2) as i32),
                           (SCREEN_WIDTH as i32, (SCREEN_HEIGHT / 2) as i32))
            .unwrap();

        renderer.set_draw_color(Color::YELLOW);
        for i in (0..SCREEN_HEIGHT).step_by(4) {
            let point = ((SCREEN_WIDTH/2) as i32, i as i32);
            renderer.draw_point(point).unwrap();
        }

        renderer.present();
    }
}