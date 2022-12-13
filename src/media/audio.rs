use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};

pub struct GameAudio {
    pub hit_block: macroquad::audio::Sound,
    pub hit_player: macroquad::audio::Sound,
    pub hit_floor: macroquad::audio::Sound,
}

impl GameAudio {
    pub async fn new() -> Self {
        Self {
            hit_block: load_sound_from_bytes(include_bytes!("../../res/audio/hit_block.wav"))
                .await
                .unwrap(),
            hit_player: load_sound_from_bytes(include_bytes!("../../res/audio/hit_player.wav"))
                .await
                .unwrap(),
            hit_floor: load_sound_from_bytes(include_bytes!("../../res/audio/hit_floor.wav"))
                .await
                .unwrap(),
        }
    }

    pub fn play_single(&self, sound: macroquad::audio::Sound) {
        let params = PlaySoundParams {
            looped: false,
            volume: 0.4,
        };
        play_sound(sound, params);
    }
}
