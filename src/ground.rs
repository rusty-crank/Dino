use core::cell::RefCell;

use playdate_rs::{
    display::DISPLAY_HEIGHT,
    graphics::{Bitmap, LCDBitmapFlip},
    math::{Point2D, Rect, Size2D},
    sprite::Sprite,
    PLAYDATE,
};

pub struct Ground {
    ground_sprites: (Sprite, Sprite),
    horizontal_velocity: RefCell<f32>,
}

impl Ground {
    pub fn new() -> Self {
        let ground = Sprite::new();
        let bitmap = Bitmap::open(2400, 24, "ground").unwrap();
        ground.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        ground.set_bounds(Rect {
            origin: Point2D::new(0.0, DISPLAY_HEIGHT as f32 - 24.0),
            size: Size2D::new(2400.0, 24.0),
        });
        ground.set_collide_rect(Rect {
            origin: Point2D::new(0.0, 18.0),
            size: Size2D::new(2400.0, 6.0),
        });
        ground.collisions_enabled();
        ground.set_z_index(-100);
        let ground2 = ground.clone();
        ground2.set_bounds(Rect {
            origin: Point2D::new(2400.0, DISPLAY_HEIGHT as f32 - 24.0),
            size: Size2D::new(2400.0, 24.0),
        });
        ground2.set_collide_rect(Rect {
            origin: Point2D::new(0.0, 18.0),
            size: Size2D::new(2400.0, 6.0),
        });
        ground2.set_z_index(-100);
        PLAYDATE.sprite.add_sprite(&ground);
        PLAYDATE.sprite.add_sprite(&ground2);
        Self {
            ground_sprites: (ground, ground2),
            horizontal_velocity: RefCell::new(0.0),
        }
    }

    pub fn update(&mut self, delta: f32) {
        // move sprites
        let mut velocity = self.horizontal_velocity.borrow_mut();
        let step = *velocity * delta;
        self.ground_sprites.0.move_by(-step, 0.0);
        self.ground_sprites.1.move_by(-step, 0.0);
        // change ground sprites order
        let pos1 = self.ground_sprites.1.get_position();
        if pos1.x <= 0.0 {
            self.ground_sprites.0.move_to(pos1.x + 2400.0, pos1.y);
            core::mem::swap(&mut self.ground_sprites.0, &mut self.ground_sprites.1);
        }
        // update velocity
        *velocity += 10.0 * delta;
    }
}
