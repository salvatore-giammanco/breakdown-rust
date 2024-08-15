use macroquad::prelude::*;

pub struct Ball {
    pub rect: Rect,
    pub vel: Vec2,
    pub speed: f32,
    pub super_ball: bool,
}

impl Ball {
    pub fn new(position: Vec2, size: f32, speed: f32) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, size, size),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
            speed,
            super_ball: false,
        }
    }

    pub fn new_super_ball(position: Vec2, size: f32, speed: f32) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, size, size),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
            speed,
            super_ball: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * self.speed;
        self.rect.y += self.vel.y * dt * self.speed;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x + self.rect.w > screen_width() {
            self.vel.x = -1f32;
        }

        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
        self.vel = self.vel.normalize();
    }

    pub fn random_direction(&mut self) {
        self.vel = vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize();
    }

    pub fn bounce(&mut self, body: &Rect) -> bool {
        if let Some(intersection) = self.rect.intersect(*body) {
            let a_center = self.rect.point() + self.rect.size() * 0.5f32;
            let b_center = body.point() + body.size() * 0.5f32;
            let to = b_center - a_center;
            let to_signum = to.signum();
            match intersection.w > intersection.h {
                true => {
                    // Bounce on y
                    self.rect.y -= to_signum.y * intersection.h;
                    self.vel.y = -to_signum.y * self.vel.y.abs();
                    // Add some randomness to x
                    self.vel.x += rand::gen_range(0f32, 0.2f32);
                }
                false => {
                    // Bounce on x
                    self.rect.x -= to_signum.x * intersection.w;
                    self.vel.x = -to_signum.x * self.vel.x.abs();
                }
            }
            return true;
        }
        false
    }

    pub fn draw(&self) {
        let color: Color = match self.super_ball {
            true => GOLD,
            false => BLUE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
