use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod game {
    use glam::Vec2;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;
    use sdl2::gfx::primitives::DrawRenderer;

    const SIZE: Vec2 = Vec2 { x: 640.0, y: 640.0 };
    const BLACK: Color = Color::RGB(0, 0, 0);
    const RED: Color = Color::RGB(255, 0, 0);

    #[derive(Copy, Clone)]
    pub enum State {
        Paused,
        Playing,
    }

    pub struct Ball {
        center: Vec2,
        velocity: Vec2,
        radius: f32
    }

    pub struct Game {
        state: State,
        ball: Ball
    }

    impl Game {
        pub fn new() -> Game {
            Game {
                state: State::Playing,
                ball: Ball {
                    center: SIZE / 2.0,
                    velocity: Vec2 { x: 0.0, y: 1.0 },
                    radius: 10.0
                }
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
            self.ball.center += self.ball.velocity;
        }

        pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
            canvas.set_draw_color(BLACK);
            canvas.clear();
            canvas.filled_circle(
                self.ball.center.x as i16,
                self.ball.center.y as i16,
                self.ball.radius as i16,
                RED
            )
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
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    let mut game = game::Game::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: u32 = 0;

    game.draw(&mut canvas).expect("draw");
    canvas.present();
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => game.toggle_state(),
                _ => {}
            }
        }

        if let game::State::Playing = game.state() {
            game.update();
            frame += 1;
        };
        game.draw(&mut canvas)?;
        canvas.present();
    }

    Ok(())
}
