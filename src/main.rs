use sdl2::{event::Event, keyboard::Keycode, controller::GameController, controller::Button};

mod game;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window("Pong", game::SIZE.x as u32, game::SIZE.y as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    let _controllers = init_game_controllers(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;

    game_loop(event_pump, canvas)?;

    Ok(())
}

fn init_game_controllers(sdl_context: &sdl2::Sdl) -> Result<Vec<GameController>, String> {
    let game_controller_subsystem = sdl_context.game_controller()?;
    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;
    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let controllers: Vec<GameController> = (0..available)
        .filter_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);
            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    println!("Opened \"{}\" with mapping {}", c.name(), c.mapping());
                    Some(c)
                }
                Err(e) => {
                    println!("Failed: {:?}", e);
                    None
                }
            }
        })
        .collect();
        println!("Initialized {} controllers", controllers.len());
        Ok(controllers)
}

fn game_loop(mut event_pump: sdl2::EventPump, mut canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    let mut game = game::Game::new();

    'running: loop {
        game.draw(&mut canvas).expect("draw");

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
                    game.handle_command(game::Command::Pause);
                },
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(command) = keycode_to_command(keycode) {
                        game.handle_command(command);
                    }
                },
                Event::ControllerButtonDown { which, button, .. } => {
                    if let Some(command) = button_to_command(which, button) {
                        game.handle_command(command);
                    }
                }
                _ => {}
            }
        }
        game.update();
    }
    Ok(())
}

fn keycode_to_command(keycode: Keycode) -> Option<game::Command> {
    match keycode {
        Keycode::A => Some(game::Command::Accelerate(0, -1.0)),
        Keycode::Z => Some(game::Command::Accelerate(0, 1.0)),
        Keycode::S => Some(game::Command::Slow(0)),
        Keycode::K => Some(game::Command::Accelerate(1, -1.0)),
        Keycode::M => Some(game::Command::Accelerate(1, 1.0)),
        Keycode::J => Some(game::Command::Slow(1)),
        _ => None,
    }
}

fn button_to_command(which: u32, button: Button) -> Option<game::Command> {
    let player_id = which as game::PlayerId;
    match button {
        Button::DPadUp => Some(game::Command::Accelerate(player_id, -1.0)),
        Button::DPadDown => Some(game::Command::Accelerate(player_id, 1.0)),
        Button::B => Some(game::Command::Slow(player_id)),
        _ => None,
    }
}