use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    Upgrade,
}

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType, block_size: f32) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, block_size, block_size),
            lives: 2,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                2 => RED,
                1 => ORANGE,
                _ => BLACK,
            },
            BlockType::Upgrade => match self.lives {
                2 => GREEN,
                1 => LIME,
                _ => BLACK,
            },
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
