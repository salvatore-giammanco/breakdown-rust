use macroquad::prelude::*;

const SCALE: f32 = 0.8;
const HEADER_POS: Vec2 = const_vec2!([30f32 * SCALE, 40f32 * SCALE]);
const FONT_SIZE: u16 = (30f32 * SCALE) as u16;
const PLAYER_SIZE: Vec2 = const_vec2!([150f32 * SCALE, 20f32 * SCALE]);
const PLAYER_SPEED: f32 = 750f32;
const PLAYER_RPOS_Y: f32 = 50f32 * SCALE;
const BLOCK_SIZE: Vec2 = const_vec2!([40f32 * SCALE, 40f32 * SCALE]);
const BALL_SIZE: f32 = 20f32 * SCALE;
const BALL_SPEED: f32 = 400f32;

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), FONT_SIZE, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams { font, font_size: FONT_SIZE, color: BLACK, ..Default::default() },
    );
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    if let Some(intersection) = a.intersect(*b) {
        let a_center = a.point() + a.size() * 0.5f32;
        let b_center = b.point() + b.size() * 0.5f32;
        let to = b_center - a_center;
        let to_signum = to.signum();
        match intersection.w > intersection.h {
            true => {
                // Bounce on y
                a.y -= to_signum.y * intersection.h;
                vel.y = -to_signum.y * vel.y.abs();
            },
            false => {
                // Bounce on x
                a.x -= to_signum.x * intersection.w;
                vel.x = -to_signum.x * vel.x.abs();
            }
        }
        return true
    }
    false
}

fn generate_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();
    let (width, height) = (15, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width()- (total_block_size.x * width as f32))* 0.5f32, 50f32);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), BlockType::Regular));
    }

    for _ in 0..3 {
        let rand_index = rand::gen_range(0, blocks.len());
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }

    return blocks;
}

pub enum GameState {
    Menu,
    Game,
    LaunchNewBall,
    LevelCompleted,
    GameOver,
}

struct Player {
    rect: Rect,
}

impl Player {
    fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
                screen_height() - PLAYER_RPOS_Y,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.x += x_move * dt * PLAYER_SPEED;

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

#[derive(PartialEq)]
enum BlockType {
    Regular,
    SpawnBallOnDeath
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block {
    fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect::new(
                pos.x,
                pos.y,
                BLOCK_SIZE.x,
                BLOCK_SIZE.y
            ),
            lives: 2,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => {
                match self.lives {
                    2 => RED,
                    1 => ORANGE,
                    _ => BLACK,
                }
            },
            BlockType::SpawnBallOnDeath => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                pos.x,
                pos.y,
                BALL_SIZE,
                BALL_SIZE
            ),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x + self.rect.w > screen_width() {
            self.vel.x = -1f32;
        }

        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }

    }


    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Game {
    state: GameState,
    player: Player,
    balls: Vec<Ball>,
    blocks: Vec<Block>,
    font: Font,
    score: i32,
    lives: i32,
}

impl Game {
    pub fn init_blocks(&mut self) {
        self.blocks = generate_blocks();
    }

    pub async fn new() -> Self {
        Self {
            state: GameState::Menu,
            player: Player::new(),
            balls: vec![Ball::new(vec2(screen_width() * 0.5f32, screen_height() - PLAYER_RPOS_Y - PLAYER_SIZE.y))],
            blocks: generate_blocks(),
            font: load_ttf_font("./res/Roboto-Regular.ttf").await.unwrap(),
            score: 0,
            lives: 3,
        }
    }

    fn new_ball_next_to_player(&self) -> Ball{
        Ball::new(self.player.rect.point() + vec2(self.player.rect.w*0.5f32-BALL_SIZE*0.5f32, -PLAYER_SIZE.y))
    }

    pub fn spawn_ball_next_to_player(&mut self) {
        self.balls.push(self.new_ball_next_to_player());
    }

    pub fn spawn_ball(&mut self, point: Vec2) {
        self.balls.push(Ball::new(point));
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.player.rect.x = screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32;
        self.balls = vec![self.new_ball_next_to_player()];
        self.init_blocks();
    }

    fn state_menu(&mut self) {
        draw_title_text("Press SPACE to start", self.font);
        if is_key_down(KeyCode::Space) {
            self.state = GameState::Game;
        }
    }

    fn state_game(&mut self){
        self.player.update(get_frame_time());
        for ball in self.balls.iter_mut() {
            ball.update(get_frame_time());
        }

        let mut spawn_later = Vec::new();
        for ball in self.balls.iter_mut() {
            resolve_collision(&mut ball.rect, &mut ball.vel, &self.player.rect);
            for block in self.blocks.iter_mut() {
                if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                    block.lives -= 1;
                    if block.lives <= 0 {
                        self.score += 10;
                        if block.block_type == BlockType::SpawnBallOnDeath {
                            spawn_later.push(ball.rect.point());
                        }
                    }
                }
            }
        }

        for point in spawn_later.into_iter() {
            self.spawn_ball(point);
        }

        let balls_len = self.balls.len();
        // Remove balls below the screen
        self.balls.retain(|ball| ball.rect.y < screen_height());

        let removed_balls = balls_len - self.balls.len();
        if removed_balls > 0 && self.balls.is_empty(){
            self.lives -= 1;
            self.state = GameState::LaunchNewBall;

            if self.lives <= 0 {
                self.state = GameState::GameOver;
            }

        }

        self.blocks.retain(|block| block.lives > 0);
        if self.blocks.is_empty() {
            self.state = GameState::LevelCompleted;
        }

        self.draw_game();
    }

    fn draw_game(&mut self){
        self.player.draw();
        for block in self.blocks.iter() {
            block.draw();
        }
        for ball in self.balls.iter() {
            ball.draw();
        }
        let score_text = format!("score: {}", self.score);
        let score_text_dim = measure_text(&score_text, Some(self.font), FONT_SIZE, 1.0);
        let text_params = TextParams { font: self.font, font_size: FONT_SIZE, color: BLACK, ..Default::default() };
        draw_text_ex(
            &score_text,
            screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
            HEADER_POS.y,
            text_params,
        );

        draw_text_ex(
            &format!("lives: {}", self.lives),
            HEADER_POS.x,
            HEADER_POS.y,
            text_params,
        );
    }

    fn state_launch_new_ball(&mut self) {
        if is_key_down(KeyCode::Space) {
            self.state = GameState::Game;
            self.spawn_ball_next_to_player();
        }
        self.draw_game();
    }

    fn state_level_completed(&mut self) {
        draw_title_text("You WIN!", self.font);
        if is_key_down(KeyCode::Space) {
            self.state = GameState::Menu;
            self.reset();
        }
    }

    fn state_game_over(&mut self) {
        draw_title_text(&format!("GAME OVER - Score: {}", self.score), self.font);
        if is_key_down(KeyCode::Space) {
            self.state = GameState::Menu;
            self.reset();
        }
    }

    pub fn frame(&mut self) {
        match self.state {
            GameState::Menu => {
                self.state_menu();
            },
            GameState::Game => {
                self.state_game();
            },
            GameState::LevelCompleted => {
                self.state_level_completed();
            },
            GameState::GameOver => {
                self.state_game_over();
            },
            GameState::LaunchNewBall => {
                self.state_launch_new_ball();
            }
        }

    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(WHITE);

        if is_key_down(KeyCode::Escape){
            break;
        }

        game.frame();
        next_frame().await
    }
}
