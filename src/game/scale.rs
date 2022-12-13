use macroquad::window::screen_width;

#[derive(Clone, Copy)]
pub struct Scale {
    pub scale: f32,
    pub screen_scale_factor: f32,
    pub screen_scale: f32,
    pub total_scale: f32,
}

impl Scale {
    pub fn new(scale: f32, screen_scale_factor: f32) -> Self {
        let screen_scale = screen_width() / screen_scale_factor;
        Self {
            scale,
            screen_scale_factor,
            screen_scale,
            total_scale: scale * screen_scale,
        }
    }

    pub fn update(&mut self) {
        self.screen_scale = screen_width() / self.screen_scale_factor;
        self.total_scale = self.scale * self.screen_scale;
    }
}
