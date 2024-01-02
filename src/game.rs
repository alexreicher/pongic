use glam::Vec2;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

pub const SIZE: Vec2 = Vec2 { x: 800.0, y: 600.0 };

const NUM_PLAYERS: usize = 2;
const PADDLE_GAP: f32 = 0.0;
const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_LENGTH: f32 = 100.0;
const BALL_RADIUS: f32 = 15.0;
const BALL_VELOCITY: f32 = 5.0;
const PADDLE_FRICTION: f32 = 5e-3;

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

struct BoundingBox {
    top_left: Vec2,
    bottom_right: Vec2
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

pub struct Game {
    ball: Ball,
    paddles: [Paddle; NUM_PLAYERS]
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..=std::f32::consts::TAU);
        Game {
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
                    color: Color::CYAN
                },
                Paddle {
                    bounding_box: BoundingBox {
                        top_left: Vec2 { x: SIZE.x - PADDLE_GAP - PADDLE_WIDTH, y: (SIZE.y - PADDLE_LENGTH) / 2.0 },
                        bottom_right: Vec2 { x: SIZE.x - PADDLE_GAP, y: (SIZE.y + PADDLE_LENGTH) / 2.0 },
                    },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    normal: Vec2 { x: -1.0, y: 0.0 },
                    color: Color::MAGENTA
                },
            ]
        }
    }

    pub fn accelerate_paddle(&mut self, player: usize, delta: f32) {
        self.paddles[player].velocity.y += delta;
    }

    pub fn update(&mut self) {
        // Move game objects
        self.ball.bounding_box.move_by(self.ball.velocity);
        for paddle in &mut self.paddles {
            paddle.bounding_box.move_by(paddle.velocity);
            paddle.velocity.y -= paddle.velocity.y.signum() * PADDLE_FRICTION;
        }

        // Check for collisions
        self.ball.bounding_box.bounds_collision(&mut self.ball.velocity);
        for paddle in &mut self.paddles {
            paddle.bounding_box.bounds_collision(&mut paddle.velocity);
            if self.ball.bounding_box.intersects(&paddle.bounding_box) && self.ball.velocity.dot(paddle.normal) < 0.0 {
                self.ball.velocity.x = -self.ball.velocity.x;
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
            self.ball.color
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

        Ok(())
    }
}
