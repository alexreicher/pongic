use glam::Vec2;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

pub const SIZE: Vec2 = Vec2 { x: 800.0, y: 600.0 };

const NUM_PLAYERS: usize = 2;
const PADDLE_GAP: f32 = 30.0;
const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_LENGTH: f32 = 100.0;

struct Ball {
    bounding_box: BoundingBox,
    velocity: Vec2,
    color: Color,
    radius: f32,
}

struct Paddle {
    bounding_box: BoundingBox,
    velocity: Vec2,
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
}

pub struct Game {
    ball: Ball,
    paddles: [Paddle; NUM_PLAYERS]
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..=std::f32::consts::TAU);
        let velocity = Vec2 {
            x: angle.cos(),
            y: angle.sin(),
        } * 5.0;

        Game {
            ball: Ball {
                bounding_box: BoundingBox {
                    top_left: Vec2 { x: SIZE.x / 2.0, y: SIZE.y / 2.0 },
                    bottom_right: Vec2 { x: SIZE.x / 2.0, y: SIZE.y / 2.0 },
                },
                velocity,
                radius: 10.0,
                color: Color::WHITE
            },
            paddles: [
                // TODO: get rid of magic numbers and duplicate code
                Paddle {
                    bounding_box: BoundingBox {
                        top_left: Vec2 { x: PADDLE_GAP, y: (SIZE.y - PADDLE_LENGTH) / 2.0 },
                        bottom_right: Vec2 { x: PADDLE_GAP + PADDLE_WIDTH, y: (SIZE.y + PADDLE_LENGTH) / 2.0 },
                    },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    color: Color::CYAN
                },
                Paddle {
                    bounding_box: BoundingBox {
                        top_left: Vec2 { x: SIZE.x - PADDLE_GAP - PADDLE_WIDTH, y: (SIZE.y - PADDLE_LENGTH) / 2.0 },
                        bottom_right: Vec2 { x: SIZE.x - PADDLE_GAP, y: (SIZE.y + PADDLE_LENGTH) / 2.0 },
                    },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    color: Color::MAGENTA
                },
            ]
        }
    }

    pub fn update(&mut self) {
        self.ball.bounding_box.move_by(self.ball.velocity);
        if self.ball.bounding_box.top_left.x - self.ball.radius < 0.0 {
            self.ball.velocity.x = -self.ball.velocity.x;
        }
        if self.ball.bounding_box.bottom_right.x + self.ball.radius > SIZE.x {
            self.ball.velocity.x = -self.ball.velocity.x;
        }
        if self.ball.bounding_box.top_left.y - self.ball.radius < 0.0 {
            self.ball.velocity.y = -self.ball.velocity.y;
        }
        if self.ball.bounding_box.bottom_right.y + self.ball.radius > SIZE.y {
            self.ball.velocity.y = -self.ball.velocity.y;
        }
        for paddle in &self.paddles {
            if self.ball.bounding_box.intersects(&paddle.bounding_box) {
                self.ball.velocity.x = -self.ball.velocity.x;
            }
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        // Draw ball
        canvas.filled_circle(
            (self.ball.bounding_box.top_left.x - self.ball.radius) as i16,
            (self.ball.bounding_box.top_left.y - self.ball.radius) as i16,
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

        canvas.present();
        Ok(())
    }
}
