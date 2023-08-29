#![no_std]

extern crate alloc;
#[macro_use]
extern crate playdate_rs;

mod animation;
mod dino;
mod ground;
mod obstacle;

use core::cell::RefCell;

use dino::Dino;
use ground::Ground;
use obstacle::Obstacles;
use playdate_rs::graphics::LCDSolidColor;
use playdate_rs::system::Buttons;
use playdate_rs::{app, println, App, PLAYDATE};

#[app]
pub struct DinoGame {
    dino: Dino,
    ground: Ground,
    obstacles: Obstacles,
    state: RefCell<GameState>,
}

impl App for DinoGame {
    fn new() -> Self {
        println!("Hello, World!");
        Self {
            dino: Dino::new(),
            ground: Ground::new(),
            obstacles: Obstacles::new(),
            state: RefCell::new(GameState::Ready),
        }
    }

    fn update(&mut self, delta: f32) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorClear);
        // Update game state
        let pushed = PLAYDATE.system.get_button_state().pushed;
        if (*self.state.borrow() == GameState::Ready || *self.state.borrow() == GameState::Dead)
            && pushed.contains(Buttons::A)
        {
            self.ground.reset();
            *self.state.borrow_mut() = GameState::Playing;
        }
        // Update and draw sprites
        self.ground.update(delta);
        self.dino.update(delta);
        self.obstacles.update(delta);
        PLAYDATE.sprite.draw_sprites();
        // Draw FPS
        PLAYDATE.system.draw_fps(vec2!(0, 0));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum GameState {
    Ready,
    Playing,
    Dead,
}
