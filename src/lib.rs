#![no_std]

extern crate alloc;
#[macro_use]
extern crate playdate_rs;

mod animation;
mod args;
mod dino;
mod ground;
mod mask;
mod obstacle;
mod scoreboard;

use core::cell::RefCell;

use dino::Dino;
use ground::Ground;
use mask::Mask;
use obstacle::Obstacles;
use playdate_rs::graphics::{Font, LCDSolidColor};
use playdate_rs::system::{Buttons, MenuItem};
use playdate_rs::{app, println, App, PLAYDATE};
use spin::Lazy;

use crate::scoreboard::Scoreboard;

const SHOW_BOUNDING_BOX: bool = false;

fn sprite_bg_color() -> LCDSolidColor {
    if SHOW_BOUNDING_BOX {
        LCDSolidColor::kColorBlack
    } else {
        LCDSolidColor::kColorClear
    }
}

#[app]
pub struct DinoGame {
    dino: Dino,
    ground: Ground,
    obstacles: Obstacles,
    mask: Mask,
    scoreboard: Scoreboard,
    state: RefCell<GameState>,
    _menu: MenuItem,
    last_invert_time_ms: usize,
    inverted: bool,
}

impl DinoGame {
    fn get_game_state() -> GameState {
        *Self::get().state.borrow()
    }

    fn is_playing(&self) -> bool {
        let state = DinoGame::get_game_state();
        state == GameState::Playing
    }

    fn is_ready_or_dead(&self) -> bool {
        let state = DinoGame::get_game_state();
        state == GameState::Ready || state == GameState::Dead
    }

    fn reset_and_start_game(&mut self) {
        self.ground.reset();
        self.dino.reset();
        self.obstacles.reset();
        self.scoreboard.reset();
        *self.state.borrow_mut() = GameState::Playing;
        self.last_invert_time_ms = PLAYDATE.system.get_current_time_milliseconds();
    }
}

impl App for DinoGame {
    fn new() -> Self {
        println!("Hello, World!");
        Self {
            dino: Dino::new(),
            ground: Ground::new(),
            obstacles: Obstacles::new(),
            mask: Mask::new(),
            scoreboard: Scoreboard::new(),
            state: RefCell::new(GameState::Ready),
            _menu: PLAYDATE
                .system
                .add_menu_item(format!("Version: {}", env!("CARGO_PKG_VERSION")), || {}),
            last_invert_time_ms: 0,
            inverted: false,
        }
    }

    fn update(&mut self, delta: f32) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorClear);
        // Update game state
        let pushed = PLAYDATE.system.get_button_state().pushed;
        if self.is_ready_or_dead() && pushed.contains(Buttons::A) {
            self.reset_and_start_game();
        }
        // Should invert the world?
        if self.is_playing() {
            let current_time = PLAYDATE.system.get_current_time_milliseconds();
            let elapsed_ms = current_time - self.last_invert_time_ms;
            if elapsed_ms > crate::args::DAY_NIGHT_CYCLE_SECS * 1000 {
                self.inverted = !self.inverted;
                self.last_invert_time_ms = current_time;
                PLAYDATE.display.set_inverted(self.inverted);
            }
        }
        // Update and draw sprites
        self.ground.update(delta);
        self.dino.update(delta);
        self.obstacles.update(delta);
        self.mask.update(delta);
        self.scoreboard.update(delta);
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

static FONT: Lazy<Font> = Lazy::new(|| {
    PLAYDATE
        .graphics
        .load_font("/System/Fonts/Roobert-10-Bold.pft")
        .unwrap()
});
