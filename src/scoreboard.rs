use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    graphics::{Bitmap, LCDBitmapFlip, LCDSolidColor},
    sprite::Sprite,
    PLAYDATE,
};

use crate::{DinoGame, GameState, FONT};

pub struct Scoreboard {
    accumulated_time: f32,
    sprite: Sprite,
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
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_time = 0.0;
    }

    fn update_sprite(&mut self) {
        let score = (self.accumulated_time * 10.0) as i32;
        let text = format!("SCORE: {:03}", score);
        let bitmap = self.sprite.get_image().unwrap();
        PLAYDATE.graphics.push_context(&bitmap);
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
        }
        self.update_sprite();
    }
}
