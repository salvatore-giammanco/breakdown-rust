use game::ball::Ball;
use game::block::{Block, BlockType};
use game::player::Player;
use game::scale::Scale;
use game::upgrades::{UpgradeCoin, UpgradeType, Upgrades};
use macroquad::prelude::*;
use media::audio::GameAudio;

mod game;
mod media;

const SCALE: f32 = 0.8;
const SCREEN_SCALE_FACTOR: f32 = 800.0;
const BLOCK_SIZE: f32 = 40.0;
const HEADER_POS: Vec2 = Vec2::from_array([5f32, 25f32]);
const FONT_SIZE: u16 = 24;
const TITLE_FONT_SIZE: u16 = 32;
const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 20f32]);
const PLAYER_SPEED: f32 = 750f32;
const PLAYER_RELATIVE_POS_Y: f32 = 50f32;
const BALL_SIZE: f32 = 20f32;
const BALL_SPEED: f32 = 400f32;

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), TITLE_FONT_SIZE, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font,
            font_size: TITLE_FONT_SIZE,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub enum GameState {
    Menu,
    Game,
    LaunchNewBall,
    LevelCompleted,
    GameOver,
}

struct Game {
    state: GameState,
    player: Player,
    balls: Vec<Ball>,
    blocks: Vec<Block>,
    font: Font,
    score: i32,
    lives: i32,
    audio: GameAudio,
    scale: Scale,
    upgrades: Upgrades,
}

impl<'a> Game {
    fn generate_blocks() -> Vec<Block> {
        let mut blocks = Vec::new();
        let (width, height) = (15, 6);
        let padding = 5f32;
        let screen_scale: f32 = screen_width() / SCREEN_SCALE_FACTOR;
        let block_size: f32 = BLOCK_SIZE * SCALE * screen_scale;
        let total_block_size = vec2(block_size, block_size) + vec2(padding, padding);
        let board_start_pos = vec2(
            (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
            50f32,
        );

        for i in 0..width * height {
            let block_x = (i % width) as f32 * total_block_size.x;
            let block_y = (i / width) as f32 * total_block_size.y;
            blocks.push(Block::new(
                board_start_pos + vec2(block_x, block_y),
                BlockType::Regular,
                block_size,
            ));
        }

        // TODO: Remove this after implementing the new upgrades
        for _ in 0..53 {
            let rand_index = rand::gen_range(0, blocks.len());
            blocks[rand_index].block_type = BlockType::Upgrade;
        }

        return blocks;
    }

    pub async fn new(scale: Scale) -> Self {
        let ball_position = vec2(
            screen_width() * 0.5f32,
            screen_height() - PLAYER_RELATIVE_POS_Y - PLAYER_SIZE.y,
        );
        Self {
            state: GameState::Menu,
            player: Player::new(PLAYER_SIZE, PLAYER_SPEED, PLAYER_RELATIVE_POS_Y, scale),
            balls: vec![Ball::new(
                ball_position,
                BALL_SIZE * scale.total_scale,
                BALL_SPEED * scale.total_scale,
            )],
            blocks: Game::generate_blocks(),
            font: load_ttf_font_from_bytes(include_bytes!("../res/Roboto-Regular.ttf")).unwrap(),
            score: 0,
            lives: 3,
            audio: GameAudio::new().await,
            scale,
            upgrades: Upgrades::new(),
        }
    }

    fn new_ball_next_to_player(&self) -> Ball {
        let ball_position = self.player.rect.point()
            + vec2(
                self.player.rect.w * 0.5f32 - BALL_SIZE * 0.5f32,
                -PLAYER_SIZE.y,
            );
        Ball::new(
            ball_position,
            BALL_SIZE * self.scale.total_scale,
            BALL_SPEED * self.scale.total_scale,
        )
    }

    pub fn spawn_ball_next_to_player(&mut self) {
        self.balls.push(self.new_ball_next_to_player());
    }

    pub fn new_super_ball_next_to_player(&self) -> Ball {
        let ball_position = self.player.rect.point()
            + vec2(
                self.player.rect.w * 0.5f32 - BALL_SIZE * 0.5f32,
                -PLAYER_SIZE.y,
            );
        Ball::new_super_ball(
            ball_position,
            BALL_SIZE * self.scale.total_scale,
            BALL_SPEED * self.scale.total_scale,
        )
    }

    pub fn spawn_ball(&mut self, point: Vec2) {
        self.balls.push(Ball::new(
            point,
            BALL_SIZE * self.scale.total_scale,
            BALL_SPEED * self.scale.total_scale,
        ));
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.scale.update();
        self.player.rect.x = screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32;
        self.balls = vec![self.new_ball_next_to_player()];
        self.blocks = Game::generate_blocks();
        self.scale.update();
        self.upgrades = Upgrades::new();
    }

    fn state_menu(&mut self) {
        draw_title_text("Press SPACE to start", self.font);
        if is_key_down(KeyCode::Space) {
            self.state = GameState::Game;
        }
    }

    fn state_game(&mut self) {
        self.player.update(get_frame_time());
        for ball in self.balls.iter_mut() {
            ball.update(get_frame_time());
        }

        let mut spawn_later = Vec::new();
        for ball in self.balls.iter_mut() {
            if ball.bounce(&self.player.rect) {
                self.audio.play_single(self.audio.hit_player);
            }
            for block in self.blocks.iter_mut() {
                if ball.bounce(&block.rect) {
                    self.audio.play_single(self.audio.hit_block);
                    if ball.super_ball {
                        block.lives = 0;
                    } else {
                        block.lives -= 1;
                    }
                    if block.lives <= 0 {
                        self.score += 10;
                        if block.block_type == BlockType::Upgrade {
                            self.upgrades.spawn_upgrade(block.rect);
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
        if removed_balls > 0 && self.balls.is_empty() {
            self.lives -= 1;
            self.audio.play_single(self.audio.hit_floor);
            self.state = GameState::LaunchNewBall;

            if self.lives <= 0 {
                self.state = GameState::GameOver;
            }
            self.upgrades.reset()
        }

        self.blocks.retain(|block| block.lives > 0);
        if self.blocks.is_empty() {
            self.state = GameState::LevelCompleted;
        }

        self.upgrades.update(self.player.rect);
        self.activate_upgrades();
        self.draw_game();
    }

    fn activate_upgrades(&'a mut self) {
        for upgrade in self.upgrades.active_upgrades.iter() {
            match upgrade {
                UpgradeType::AddBall => {
                    self.balls.push(self.new_ball_next_to_player());
                }
                UpgradeType::ExtraLife => {
                    self.lives += 1;
                }
                UpgradeType::SuperBall => {
                    self.balls.push(self.new_super_ball_next_to_player());
                }
                UpgradeType::BallMultiplier => {
                    let mut new_balls: Vec<Ball> = vec![];
                    for ball in self.balls.iter() {
                        let mut new_ball: Ball = match ball. super_ball{
                            true => Ball::new_super_ball(ball.rect.point(), ball.rect.w, ball.speed),
                            false => Ball::new(ball.rect.point(), ball.rect.w, ball.speed),
                        };
                        new_ball.random_direction();
                        new_balls.push(new_ball);
                    }
                    for ball in new_balls {
                        self.balls.push(ball);
                    }
                },
                UpgradeType::SpaceInvader => {
                    self.upgrades.space_invader_active = true;
                },
                UpgradeType::Magnet => {
                    self.upgrades.magnet_active = true;
                },
                _ => {
                    self.balls.push(self.new_ball_next_to_player());
                }
            }
        }
        self.upgrades.active_upgrades.clear();
    }

    fn draw_game(&mut self) {
        self.player.draw();
        for block in self.blocks.iter() {
            block.draw();
        }
        for ball in self.balls.iter() {
            ball.draw();
        }
        self.upgrades.draw();
        let score_text = format!("score: {}", self.score);
        let final_font_size = (FONT_SIZE as f32 * self.scale.total_scale) as u16;
        let score_text_dim = measure_text(&score_text, Some(self.font), final_font_size, 1.0);
        let text_params = TextParams {
            font: self.font,
            font_size: final_font_size,
            color: BLACK,
            ..Default::default()
        };
        draw_text_ex(
            &score_text,
            screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
            HEADER_POS.y * self.scale.total_scale,
            text_params,
        );

        draw_text_ex(
            &format!("lives: {}", self.lives),
            HEADER_POS.x,
            HEADER_POS.y * self.scale.total_scale,
            text_params,
        );
    }

    fn state_launch_new_ball(&mut self) {
        self.player.update(get_frame_time());
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
            }
            GameState::Game => {
                self.state_game();
            }
            GameState::LevelCompleted => {
                self.state_level_completed();
            }
            GameState::GameOver => {
                self.state_game_over();
            }
            GameState::LaunchNewBall => {
                self.state_launch_new_ball();
            }
        }
    }
}

#[macroquad::main("Breakdown")]
async fn main() {
    let scale: Scale = Scale::new(SCALE, SCREEN_SCALE_FACTOR);
    let mut game = Game::new(scale).await;

    loop {
        clear_background(WHITE);

        if is_key_down(KeyCode::Escape) {
            break;
        }

        game.frame();
        next_frame().await
    }
}
