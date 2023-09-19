use crate::FONT;
use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    graphics::{Bitmap, BitmapFlip, Color},
    math::Vec2,
    sprite::Sprite,
    PLAYDATE,
};

use crate::{DinoGame, GameState};

const CENTER: Vec2<f32> = vec2!(x: DISPLAY_WIDTH as f32 / 2.0, y: DISPLAY_HEIGHT as f32 / 2.0);

pub struct UILayer {
    start_message: MessageBox,
    restart_panel: RestartPanel,
}

impl UILayer {
    pub fn new() -> Self {
        Self {
            start_message: MessageBox::new("Press Ⓐ to start", CENTER),
            restart_panel: RestartPanel::new(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        let game_state = DinoGame::get_game_state();
        self.start_message
            .update(delta, game_state == GameState::Ready);
        self.restart_panel
            .update(delta, game_state == GameState::Dead);
    }
}

struct MessageBox {
    sprite: Sprite,
    elapsed: f32,
}

impl MessageBox {
    pub fn new(text: impl AsRef<str>, center: Vec2<f32>) -> Self {
        // Get text size
        let text_width = FONT.get_text_width(&text, 0) as i32;
        let text_height = FONT.get_height() as i32;
        // Draw text to a bitmap
        let bitmap = Bitmap::new(size!(text_width as _, text_height as _), Color::White);
        PLAYDATE.graphics.push_context(&bitmap);
        PLAYDATE.graphics.set_font(&FONT);
        PLAYDATE.graphics.draw_text(text, vec2!(0, 0));
        PLAYDATE.graphics.pop_context();
        // Create sprite
        let sprite = Sprite::new();
        sprite.set_image(bitmap, BitmapFlip::Unflipped);
        sprite.set_z_index(20000);
        sprite.set_bounds(rect!(x: center.x - text_width as f32 / 2.0, y: center.y - text_height as f32 / 2.0, w: text_width as _, h: text_height as _));
        PLAYDATE.sprite.add_sprite(&sprite);
        Self {
            sprite,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32, visible: bool) {
        if !visible {
            self.sprite.set_visible(visible);
            return;
        }
        self.elapsed += delta;
        if (self.elapsed as i32) & 1 == 1 {
            self.sprite.set_visible(false);
        } else {
            self.sprite.set_visible(true);
        }
    }
}

struct RestartPanel {
    game_over_image: Sprite,
    restart_icon: Sprite,
    message: MessageBox,
}

impl RestartPanel {
    fn new() -> Self {
        // Create game over icon
        let game_over_image = Sprite::new();
        let bitmap = Bitmap::open("game-over").unwrap();
        let bitmap_scaled = Bitmap::new(size!(195, 15), Color::Clear);
        PLAYDATE.graphics.push_context(&bitmap_scaled);
        PLAYDATE
            .graphics
            .draw_scaled_bitmap(&bitmap, vec2!(0, 0), vec2!(0.5, 0.5));
        PLAYDATE.graphics.pop_context();
        game_over_image.set_image(bitmap_scaled, BitmapFlip::Unflipped);
        game_over_image.set_z_index(10000);
        game_over_image.set_bounds(
            rect!(x: CENTER.x - 195.0 / 2.0, y:  CENTER.y - 16.0 - 20.0 - 30.0 , w: 390.0, h: 30.0),
        );
        PLAYDATE.sprite.add_sprite(&game_over_image);
        // Create restart icon
        let restart_icon = Sprite::new();
        let bitmap = Bitmap::open("restart").unwrap();
        let bitmap_scaled = Bitmap::new(size!(36, 32), Color::Clear);
        PLAYDATE.graphics.push_context(&bitmap_scaled);
        PLAYDATE
            .graphics
            .draw_scaled_bitmap(&bitmap, vec2!(0, 0), vec2!(0.5, 0.5));
        PLAYDATE.graphics.pop_context();
        restart_icon.set_image(bitmap_scaled, BitmapFlip::Unflipped);
        restart_icon.set_z_index(10000);
        restart_icon
            .set_bounds(rect!(x: CENTER.x - 18.0, y:  CENTER.y - 16.0 - 20.0, w: 36.0, h: 32.0));
        PLAYDATE.sprite.add_sprite(&restart_icon);
        Self {
            game_over_image,
            restart_icon,
            message: MessageBox::new("Press Ⓐ to restart", CENTER + vec2!(0.0, 18.0)),
        }
    }

    fn update(&mut self, delta: f32, visible: bool) {
        self.game_over_image.set_visible(visible);
        self.restart_icon.set_visible(visible);
        self.message.update(delta, visible);
    }
}
