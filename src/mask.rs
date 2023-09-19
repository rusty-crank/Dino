use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    graphics::{Bitmap, BitmapFlip, Color},
    math::Size,
    sprite::Sprite,
    PLAYDATE,
};

use crate::{DinoGame, GameState};

const MASK_SIZE: Size<f32> = size!(DISPLAY_WIDTH as f32 - 80.0, DISPLAY_HEIGHT as f32);

pub struct Mask {
    left_sprite: Sprite,
    right_sprite: Sprite,
}

impl Mask {
    pub fn new() -> Self {
        // Create left and right mask
        let bitmap = Bitmap::new(
            size!(MASK_SIZE.width as _, MASK_SIZE.height as _),
            Color::White,
        );
        let right_sprite = Sprite::new();
        right_sprite.set_image(bitmap, BitmapFlip::Unflipped);
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
        }
    }

    pub fn update(&mut self, delta: f32) {
        let game_state = DinoGame::get_game_state();
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
    }
}
