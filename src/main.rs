use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use glam::Vec2;

mod pong {
    #[derive(Copy, Clone)]
    pub enum State {
        Paused,
        Playing,
    }

    pub struct Pong {
        state: State
    }

    impl Pong {
        pub fn new() -> Pong {
            Pong {
                state: State::Paused,
            }
        }

        pub fn toggle_state(&mut self) {
            self.state = match self.state {
                State::Paused => State::Playing,
                State::Playing => State::Paused,
            }
        }

        pub fn state(&self) -> State {
            self.state
        }

        pub fn update(&mut self) {
        }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window("Pong", 640, 640)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    // this struct manages textures. For lifetime reasons, the canvas cannot directly create
    // textures, you have to create a `TextureCreator` instead.
    // let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let mut game = pong::Pong::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: u32 = 0;
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // update the game loop here
        if frame >= 3 {
            game.update();
            frame = 0;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        if let pong::State::Playing = game.state() {
            frame += 1;
        };
    }

    Ok(())
}
