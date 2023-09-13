use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    fs::{File, Write},
    graphics::{Bitmap, LCDBitmapFlip, LCDSolidColor},
    sound::FilePlayer,
    sprite::Sprite,
    sys::FileOptions,
    PLAYDATE,
};

use crate::{DinoGame, GameState, FONT};

pub struct Scoreboard {
    accumulated_time: f32,
    sprite: Sprite,
    achievement_audio: FilePlayer,
    record: MaxRecord,
}

impl Scoreboard {
    pub fn new() -> Self {
        let bitmap = Bitmap::new(
            size!(DISPLAY_WIDTH as _, DISPLAY_HEIGHT as _),
            LCDSolidColor::kColorClear,
        );
        let sprite = Sprite::new();
        sprite.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        sprite.set_z_index(10000);
        sprite.set_bounds(rect!(x: 0.0, y: 0.0, w: DISPLAY_WIDTH as _, h: DISPLAY_HEIGHT as _));
        PLAYDATE.sprite.add_sprite(&sprite);
        Self {
            accumulated_time: 0.0,
            sprite,
            achievement_audio: FilePlayer::open("achievement").unwrap(),
            record: MaxRecord::new(),
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_time = 0.0;
    }

    fn get_score(&self) -> i32 {
        (self.accumulated_time * 10.0) as i32
    }

    fn update_sprite(&mut self) {
        let score = self.get_score();
        let text = format!("HI {:05} {:05}", self.record.get(), score);
        let bitmap = self.sprite.get_image().unwrap();
        PLAYDATE.graphics.push_context(bitmap);
        PLAYDATE.graphics.clear(crate::sprite_bg_color());
        PLAYDATE.graphics.set_font(&FONT);
        PLAYDATE.graphics.draw_text(
            &text,
            vec2!(
                DISPLAY_WIDTH as i32 - FONT.get_text_width(&text, 0) as i32 - 2,
                0
            ),
        );
        PLAYDATE.graphics.pop_context();
    }

    pub fn update(&mut self, delta: f32) {
        let state = DinoGame::get_game_state();
        if state == GameState::Playing {
            self.accumulated_time += delta;
            let score = self.get_score();
            if DinoGame::enable_audio() && score > 0 && score % 100 == 0 {
                self.achievement_audio.play(1);
            }
        }
        if state == GameState::Dead {
            self.record.update(self.get_score());
        }
        self.update_sprite();
    }
}

struct MaxRecord {
    value: i32,
}

impl MaxRecord {
    pub fn new() -> Self {
        let value = File::open("record", FileOptions::kFileReadData)
            .map(|mut f| f.read_to_string().unwrap().parse::<i32>().unwrap())
            .unwrap_or_default();
        Self { value }
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn update(&mut self, score: i32) {
        if score > self.value {
            self.value = score;
            let mut file = File::open("record", FileOptions::kFileWrite).unwrap();
            let s = format!("{}", self.value);
            file.write_all(s.as_bytes()).unwrap();
        }
    }
}
