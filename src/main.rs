use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod game;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Paused,
    Playing,
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
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    let mut game = game::Game::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut state: State = State::Playing;
    'running: loop {
        game.draw(&mut canvas).expect("draw");
        canvas.present();

        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    state = match state {
                        State::Paused => State::Playing,
                        State::Playing => State::Paused,
                    }
                }
                _ => {}
            }
        }
        if state == State::Playing {
            game.update();
        };
    }

    Ok(())
}
