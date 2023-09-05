use crate::FONT;
use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    graphics::{Bitmap, LCDBitmapFlip, LCDSolidColor},
    math::Size,
    sprite::Sprite,
    PLAYDATE,
};

use crate::{DinoGame, GameState};

const MASK_SIZE: Size<f32> = size!(DISPLAY_WIDTH as f32 - 80.0, DISPLAY_HEIGHT as f32);

struct MessageBox {
    sprite: Sprite,
    displaying: bool,
    elapsed: f32,
}

impl MessageBox {
    pub fn new(text: impl AsRef<str>) -> Self {
        let bitmap = Bitmap::new(
            size!(DISPLAY_WIDTH as _, DISPLAY_HEIGHT as _),
            LCDSolidColor::kColorClear,
        );
        PLAYDATE.graphics.push_context(&bitmap);
        let text_width = FONT.get_text_width(&text, 0) as i32;
        let text_height = FONT.get_height() as i32;
        PLAYDATE.graphics.fill_rect(
            rect!(
                x: (DISPLAY_WIDTH as i32 - text_width) / 2 - 10,
                y: (DISPLAY_HEIGHT as i32 - text_height * 3) / 2 - 10,
                w: text_width + 20,
                h: text_height  + 20
            ),
            LCDSolidColor::kColorWhite,
        );
        PLAYDATE.graphics.set_font(&FONT);
        PLAYDATE.graphics.draw_text(
            text,
            vec2!(
                (DISPLAY_WIDTH as i32 - text_width) / 2,
                (DISPLAY_HEIGHT as i32 - text_height * 3) / 2
            ),
        );
        PLAYDATE.graphics.pop_context();
        let sprite = Sprite::new();
        sprite.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        sprite.set_z_index(20000);
        sprite.set_bounds(rect!(x: 0.0, y: 0.0, w: DISPLAY_WIDTH as _, h: DISPLAY_HEIGHT as _));
        Self {
            sprite,
            displaying: false,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, visible: bool, delta: f32) {
        if !visible {
            if !self.displaying {
                return;
            }
            self.displaying = false;
            PLAYDATE.sprite.remove_sprite(&self.sprite);
            return;
        }

        self.elapsed += delta;
        if (self.elapsed as i32) & 1 == 1 {
            self.sprite.set_visible(false);
        } else {
            self.sprite.set_visible(true);
        }
        if self.displaying {
            return;
        }
        self.displaying = true;
        PLAYDATE.sprite.add_sprite(&self.sprite);
    }
}

pub struct Mask {
    left_sprite: Sprite,
    right_sprite: Sprite,
    start_message: MessageBox,
    restart_message: MessageBox,
}

impl Mask {
    pub fn new() -> Self {
        let bitmap = Bitmap::new(
            size!(MASK_SIZE.width as _, MASK_SIZE.height as _),
            LCDSolidColor::kColorWhite,
        );
        let right_sprite = Sprite::new();
        right_sprite.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        right_sprite.set_z_index(10000);
        right_sprite.set_bounds(rect!(x: 100.0, y: 0.0, w: MASK_SIZE.width, h: MASK_SIZE.height));
        PLAYDATE.sprite.add_sprite(&right_sprite);
        let left_sprite = right_sprite.clone();
        left_sprite.set_bounds(
            rect!(x: 20.0 - MASK_SIZE.width, y: 0.0, w: MASK_SIZE.width, h: MASK_SIZE.height),
        );
        PLAYDATE.sprite.add_sprite(&left_sprite);
        Self {
            left_sprite,
            right_sprite,
            start_message: MessageBox::new("Press (A) to start"),
            restart_message: MessageBox::new("Press (A) to restart"),
        }
    }

    pub fn update(&mut self, delta: f32) {
        let game_state = DinoGame::get_game_state();

        self.start_message
            .update(game_state == GameState::Ready, delta);
        self.restart_message
            .update(game_state == GameState::Dead, delta);

        if game_state != GameState::Playing {
            return;
        }
        // reveal the main scene
        let pos = self.left_sprite.get_position();
        if pos.x + MASK_SIZE.width / 2.0 > 0.0 {
            self.left_sprite.move_by(vec2!(x: -500.0 * delta, y: 0.0));
        }
        let pos = self.right_sprite.get_position();
        if pos.x - MASK_SIZE.width / 2.0 < DISPLAY_WIDTH as f32 {
            self.right_sprite.move_by(vec2!(x: 500.0 * delta, y: 0.0));
        }
        // display messages
        // PLAYDATE.graphics.draw_text(text, 0, 0);
    }
}
