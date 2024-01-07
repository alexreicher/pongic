use glam::Vec2;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

pub const SIZE: Vec2 = Vec2 { x: 800.0, y: 600.0 };

const NUM_PLAYERS: usize = 2;
const BALL_RADIUS: f32 = 15.0;
const BALL_VELOCITY: f32 = 5.0;
const PADDLE_GAP: f32 = 0.0;
const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_LENGTH: f32 = 100.0;
const PADDLE_FRICTION: f32 = 1e-2;
const PADDLE_ACCELERATION: f32 = 2.0;
const PADDLE_DECAY: f32 = 1e-2;
const PADDLE_BONUS: f32 = 10.0;

struct Ball {
    bounding_box: BoundingBox,
    velocity: Vec2,
    color: Color,
    radius: f32,
}

struct Paddle {
    bounding_box: BoundingBox,
    velocity: Vec2,
    normal: Vec2,
    color: Color
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Paused,
    Playing,
    Victory(usize)
}

struct BoundingBox {
    top_left: Vec2,
    bottom_right: Vec2
}
pub struct Game {
    state: State,
    ball: Ball,
    paddles: [Paddle; NUM_PLAYERS]
}

#[derive(Copy, Clone)]
pub enum Command {
    Pause,
    Accelerate(usize, f32),
    Slow(usize)
}

impl BoundingBox {
    fn intersects(&self, other: &BoundingBox) -> bool {
        self.top_left.x < other.bottom_right.x &&
        self.bottom_right.x > other.top_left.x &&
        self.top_left.y < other.bottom_right.y &&
        self.bottom_right.y > other.top_left.y
    }

    fn move_by(&mut self, delta: Vec2) {
        self.top_left += delta;
        self.bottom_right += delta;
    }

    fn bounds_collision(&self, velocity: &mut Vec2) {
        if self.top_left.x < 0.0 {
            velocity.x = -velocity.x;
        }
        if self.bottom_right.x > SIZE.x {
            velocity.x = -velocity.x;
        }
        if self.top_left.y < 0.0 {
            velocity.y = -velocity.y;
        }
        if self.bottom_right.y > SIZE.y {
            velocity.y = -velocity.y;
        }
    }
}

impl Paddle {
    fn move_by(&mut self, delta: Vec2) {
        self.bounding_box.move_by(delta);
    }

    fn expand_by(&mut self, delta: f32) {
        self.bounding_box.top_left.y -= delta / 2.0;
        self.bounding_box.bottom_right.y += delta / 2.0;
    }

    fn length(&self) -> f32 {
        self.bounding_box.bottom_right.y - self.bounding_box.top_left.y
    }
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..=std::f32::consts::TAU);
        Game {
            state: State::Playing,
            ball: Ball {
                bounding_box: BoundingBox {
                    top_left: SIZE / 2.0 - Vec2 { x: BALL_RADIUS, y: BALL_RADIUS },
                    bottom_right: SIZE / 2.0 + Vec2 { x: BALL_RADIUS, y: BALL_RADIUS },
                },
                velocity: Vec2 { x: angle.cos(), y: angle.sin() } * BALL_VELOCITY,
                radius: BALL_RADIUS,
                color: Color::YELLOW
            },
            paddles: [
                // TODO: get rid of duplicate code
                Paddle {
                    bounding_box: BoundingBox {
                        top_left: Vec2 { x: PADDLE_GAP, y: (SIZE.y - PADDLE_LENGTH) / 2.0 },
                        bottom_right: Vec2 { x: PADDLE_GAP + PADDLE_WIDTH, y: (SIZE.y + PADDLE_LENGTH) / 2.0 },
                    },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    normal: Vec2 { x: 1.0, y: 0.0 },
                    color: Color::GREEN
                },
                Paddle {
                    bounding_box: BoundingBox {
                        top_left: Vec2 { x: SIZE.x - PADDLE_GAP - PADDLE_WIDTH, y: (SIZE.y - PADDLE_LENGTH) / 2.0 },
                        bottom_right: Vec2 { x: SIZE.x - PADDLE_GAP, y: (SIZE.y + PADDLE_LENGTH) / 2.0 },
                    },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    normal: Vec2 { x: -1.0, y: 0.0 },
                    color: Color::BLUE
                },
            ]
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Pause => self.toggle_pause(),
            Command::Accelerate(player, signum) => self.accelerate(player, signum),
            Command::Slow(player) => self.slow(player)
        }
    }

    fn toggle_pause(&mut self) {
        match self.state {
            State::Paused => self.state = State::Playing,
            State::Playing => self.state = State::Paused,
            _ => {}
        }
    }

    fn accelerate(&mut self, player: usize, signum: f32) {
        self.paddles[player].velocity.y += PADDLE_ACCELERATION * signum;
    }

    fn slow(&mut self, player: usize) {
        self.paddles[player].velocity.y *= 0.75;
    }

    pub fn update(&mut self) {
        if self.state != State::Playing {
            return;
        }

        // Move game objects
        self.ball.bounding_box.move_by(self.ball.velocity);
        for paddle in &mut self.paddles {
            paddle.move_by(paddle.velocity);
            paddle.velocity.y -= paddle.velocity.y.signum() * PADDLE_FRICTION;
            paddle.expand_by(-PADDLE_DECAY);
        }

        // Check for collisions
        self.ball.bounding_box.bounds_collision(&mut self.ball.velocity);
        for paddle in &mut self.paddles {
            paddle.bounding_box.bounds_collision(&mut paddle.velocity);
            if self.ball.bounding_box.intersects(&paddle.bounding_box) && self.ball.velocity.dot(paddle.normal) < 0.0 {
                self.ball.velocity.x = -self.ball.velocity.x;
                paddle.expand_by(PADDLE_BONUS);
            }
        }

        // Check for victory
        for i in 0..NUM_PLAYERS {
            let length = self.paddles[i].length();
            if length > PADDLE_LENGTH * 2.0 {
                self.state = State::Victory(i)
            } else if length < PADDLE_WIDTH {
                self.state = State::Victory((i + 1) % NUM_PLAYERS)
            }
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        // Clear screen
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw ball
        canvas.filled_circle(
            (self.ball.bounding_box.top_left.x + self.ball.radius) as i16,
            (self.ball.bounding_box.top_left.y + self.ball.radius) as i16,
            self.ball.radius as i16,
            match self.state {
                State::Paused => Color::GRAY,
                State::Victory(winner) => self.paddles[winner].color,
                _ => self.ball.color
            }
        )?;

        // Draw paddles
        for paddle in &self.paddles {
            canvas.set_draw_color(paddle.color);
            canvas.fill_rect(sdl2::rect::Rect::new(
                paddle.bounding_box.top_left.x as i32,
                paddle.bounding_box.top_left.y as i32,
                (paddle.bounding_box.bottom_right.x - paddle.bounding_box.top_left.x) as u32,
                (paddle.bounding_box.bottom_right.y - paddle.bounding_box.top_left.y) as u32
            ))?;
        }

        canvas.present();
        Ok(())
    }
}
