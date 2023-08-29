#![no_std]

extern crate alloc;

mod animation;
mod dino;
mod ground;

use core::cell::RefCell;

use dino::Dino;
use ground::Ground;
use playdate_rs::graphics::LCDSolidColor;
use playdate_rs::system::Buttons;
use playdate_rs::{app, println, App, PLAYDATE};

#[app]
pub struct DinoGame {
    dino: Dino,
    ground: Ground,
    state: RefCell<GameState>,
}

impl App for DinoGame {
    fn new() -> Self {
        println!("Hello, World!");
        Self {
            dino: Dino::new(),
            ground: Ground::new(),
            state: RefCell::new(GameState::Ready),
        }
    }

    fn update(&mut self, delta: f32) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorClear);
        // Update game state
        let pushed = PLAYDATE.system.get_button_state().pushed;
        if *self.state.borrow() == GameState::Ready || *self.state.borrow() == GameState::Dead {
            if pushed.contains(Buttons::A) {
                self.ground.reset();
                *self.state.borrow_mut() = GameState::Playing;
            }
        }
        // Update and draw sprites
        self.ground.update(delta);
        self.dino.update(delta);
        PLAYDATE.sprite.draw_sprites();
        // Draw FPS
        PLAYDATE.system.draw_fps(0, 0);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum GameState {
    Ready,
    Playing,
    Dead,
}
