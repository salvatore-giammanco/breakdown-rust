use macroquad::prelude::*;
pub struct Player {
    pub rect: Rect,
    speed: f32,
}

impl Player {
    pub fn new(size: Vec2, speed: f32, relative_pos_y: f32) -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - size.x * 0.5f32,
                screen_height() - relative_pos_y,
                size.x,
                size.y
            ),
            speed
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.x += x_move * dt * self.speed;

        if self.rect.x <= 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x + self.rect.w >= screen_width() {
            self.rect.x = screen_width() - self.rect.w;
        }

    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}