use glam::Vec2;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

const SIZE: Vec2 = Vec2 { x: 640.0, y: 640.0 };
const BLACK: Color = Color::RGB(0, 0, 0);
const RED: Color = Color::RGB(255, 0, 0);

struct Ball {
    center: Vec2,
    velocity: Vec2,
    radius: f32
}

pub struct Game {
    ball: Ball
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..=std::f32::consts::TAU);
        let velocity = Vec2 {
            x: angle.cos(),
            y: angle.sin(),
        };

        Game {
            ball: Ball {
                center: SIZE / 2.0,
                velocity,
                radius: 10.0,
            },
        }
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
