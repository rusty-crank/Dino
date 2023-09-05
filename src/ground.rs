use core::cell::RefCell;

use playdate_rs::{
    display::DISPLAY_HEIGHT,
    graphics::{Bitmap, LCDBitmapFlip},
    sprite::Sprite,
    App, PLAYDATE,
};

use crate::{DinoGame, GameState};

pub struct Ground {
    pub ground_sprites: (Sprite, Sprite),
    horizontal_velocity: RefCell<f32>,
}

impl Ground {
    const HEIGHT: f32 = 64.0;
    pub const COLLIDE_HEIGHT: f32 = 64.0 - 18.0;

    pub fn new() -> Self {
        let ground = Sprite::new();
        let bitmap = Bitmap::open(size!(2400, 24), "ground").unwrap();
        ground.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        ground.set_collide_rect(rect!(x: 0.0, y: 18.0, w: 2400.0, h: Self::HEIGHT));
        ground.collisions_enabled();
        ground.set_z_index(-100);
        let ground2 = ground.clone();
        ground2.set_collide_rect(rect!(x: 0.0, y: 18.0, w: 2400.0, h: Self::HEIGHT));
        ground2.set_z_index(-100);
        PLAYDATE.sprite.add_sprite(&ground);
        PLAYDATE.sprite.add_sprite(&ground2);
        let mut ground = Self {
            ground_sprites: (ground, ground2),
            horizontal_velocity: RefCell::new(0.0),
        };
        ground.reset();
        ground
    }

    pub fn reset(&mut self) {
        let y = DISPLAY_HEIGHT as f32 - Self::HEIGHT;
        self.ground_sprites
            .0
            .set_bounds(rect!(x: 0.0, y: y, w: 2400.0, h: 24.0));
        self.ground_sprites
            .1
            .set_bounds(rect!(x: 2400.0, y: y, w: 2400.0, h: 24.0));
        *self.horizontal_velocity.borrow_mut() = 10.0;
    }

    pub fn sprite_is_ground(&self, s: &Sprite) -> bool {
        s == &self.ground_sprites.0 || s == &self.ground_sprites.1
    }

    pub fn get_velocity(&self) -> f32 {
        *self.horizontal_velocity.borrow()
    }

    pub fn update(&mut self, delta: f32) {
        if *DinoGame::get().state.borrow() != GameState::Playing {
            return;
        }
        // move sprites
        let mut velocity = self.horizontal_velocity.borrow_mut();
        let step = *velocity * delta;
        self.ground_sprites.0.move_by(vec2![-step, 0.0]);
        self.ground_sprites.1.move_by(vec2![-step, 0.0]);
        // change ground sprites order
        let pos1 = self.ground_sprites.1.get_position();
        if pos1.x <= 0.0 {
            self.ground_sprites
                .0
                .move_to(vec2![pos1.x + 2400.0, pos1.y]);
            core::mem::swap(&mut self.ground_sprites.0, &mut self.ground_sprites.1);
        }
        // update velocity
        *velocity += 10.0 * delta;
    }
}
