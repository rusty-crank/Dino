use alloc::{vec, vec::Vec};
use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    graphics::{Bitmap, LCDBitmapFlip},
    rand::Rng,
    sprite::Sprite,
    App, PLAYDATE,
};

use crate::{DinoGame, GameState};

pub struct BGItem {
    sprite: Sprite,
}

impl BGItem {
    pub fn new() -> Self {
        // Create cloud image
        let pos_x = DISPLAY_WIDTH as f32
            + playdate_rs::util::rand::rng().gen_range(0.0..=DISPLAY_WIDTH as f32 - 46.0 - 32.0);
        let pos_y = playdate_rs::util::rand::rng().gen_range(32.0..=DISPLAY_HEIGHT as f32 / 2.0);
        let cloud = Sprite::new();
        let bitmap = Bitmap::open(size!(46, 14), "cloud").unwrap();
        cloud.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        cloud.set_z_index(-100);
        cloud.set_bounds(rect!(x: pos_x, y: pos_y, w: 46.0, h: 14.0));
        PLAYDATE.sprite.add_sprite(&cloud);
        Self { sprite: cloud }
    }

    pub fn update(&mut self, delta: f32) {
        if DinoGame::get_game_state() != GameState::Playing {
            return;
        }
        let pos = self.sprite.get_position();
        let velocity = DinoGame::get().ground.get_velocity() * 0.3;
        self.sprite
            .move_to(vec2!(x: pos.x - velocity * delta, y: pos.y));
    }
}

impl Drop for BGItem {
    fn drop(&mut self) {
        PLAYDATE.sprite.remove_sprite(&self.sprite);
    }
}

pub struct BGItems {
    items: Vec<BGItem>,
}

impl BGItems {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn reset(&mut self) {
        self.items.clear();
    }

    pub fn update(&mut self, delta: f32) {
        // Update items
        for obstacle in &mut self.items {
            obstacle.update(delta);
        }
        // remove items that are off screen
        self.items.retain(|x| {
            let rect = x.sprite.get_bounds();
            rect.x + rect.width >= 0.0
        });
        // If there are no items to the right of the screen, add new ones
        let has_hidden_items = self.items.iter().any(|x| {
            let rect = x.sprite.get_bounds();
            let x_right = rect.x + rect.width;
            x_right > DISPLAY_WIDTH as f32
        });
        if !has_hidden_items {
            println!("new item");
            self.items.push(BGItem::new());
        }
    }
}
