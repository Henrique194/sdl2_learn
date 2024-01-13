pub mod lazy_foo {
    mod create_window;
    pub use create_window::run as create_window;

    mod image_screen;
    pub use image_screen::run as image_screen;

    mod event_driven;
    pub use event_driven::run as event_driven;

    mod key_presses;
    pub use key_presses::run as key_presses;

    mod optimized_surface;
    pub use optimized_surface::run as optimized_surface;

    mod loading_other_image;
    pub use loading_other_image::run as loading_other_image;

    mod texture_loading;
    pub use texture_loading::run as texture_loading;

    mod geometry_rendering;
    pub use geometry_rendering::run as geometry_rendering;

    mod gamepads_and_joysticks;
    pub use gamepads_and_joysticks::run as gamepads_and_joysticks;
}
