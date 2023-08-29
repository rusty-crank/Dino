#![no_std]

extern crate alloc;

mod animation;
mod dino;
mod ground;

use dino::Dino;
use ground::Ground;
use playdate_rs::graphics::LCDSolidColor;
use playdate_rs::{app, println, App, PLAYDATE};

#[app]
pub struct HelloWorld {
    dino: Dino,
    ground: Ground,
}

impl App for HelloWorld {
    fn new() -> Self {
        println!("Hello, World!");
        Self {
            dino: Dino::new(),
            ground: Ground::new(),
        }
    }

    fn update(&mut self, delta: f32) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite);
        // Update and draw sprites
        self.ground.update(delta);
        self.dino.update(delta);
        PLAYDATE.sprite.draw_sprites();
        // Draw FPS
        PLAYDATE.system.draw_fps(0, 0);
    }
}
