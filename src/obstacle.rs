use alloc::{sync::Arc, vec, vec::Vec};
use playdate_rs::{
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    graphics::{Bitmap, BitmapTable, LCDBitmapFlip, LCDSolidColor},
    rand::Rng,
    sprite::Sprite,
    App, PLAYDATE,
};

use crate::{animation::Animation, ground::Ground, DinoGame, GameState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ObstacleKind {
    Bird,
    CactusSmall1,
    CactusSmall2,
    CactusSmall3,
    CactusBig1,
    CactusBig2,
    CactusBig3,
}

impl ObstacleKind {
    fn random() -> Self {
        let kinds = [
            ObstacleKind::Bird,
            ObstacleKind::CactusSmall1,
            ObstacleKind::CactusSmall2,
            ObstacleKind::CactusSmall3,
            ObstacleKind::CactusBig1,
            ObstacleKind::CactusBig2,
            ObstacleKind::CactusBig3,
        ];
        let i: usize = PLAYDATE.system.rand().gen_range(0..kinds.len());
        kinds[i]
        // ObstacleKind::Bird
    }
}

pub struct ObstacleImages {
    bird: Arc<BitmapTable>,
    cactus_small1: Bitmap,
    cactus_small2: Bitmap,
    cactus_small3: Bitmap,
    cactus_big1: Bitmap,
    cactus_big2: Bitmap,
    cactus_big3: Bitmap,
}

pub struct Obstacle {
    kind: ObstacleKind,
    anim: Option<Animation>,
    sprite: Sprite,
}

impl Obstacle {
    pub fn new(images: &ObstacleImages) -> Self {
        let mut me = Self {
            kind: ObstacleKind::random(),
            anim: None,
            sprite: Sprite::new(),
        };
        me.gen_sprite(images);
        me
    }

    fn gen_sprite(&mut self, images: &ObstacleImages) {
        let pos_x = DISPLAY_WIDTH as f32;
        const BIRD_Y: f32 = 88.0;
        println!("Create {:?}", self.kind);
        match self.kind {
            ObstacleKind::Bird => {
                let image = Bitmap::new(46, 34, LCDSolidColor::kColorClear);
                self.sprite
                    .set_image(image, LCDBitmapFlip::kBitmapUnflipped);
                self.sprite
                    .set_bounds(rect!(x: pos_x, y: BIRD_Y, w: 46.0, h: 34.0));
                // self.sprite
                //     .set_collide_rect(rect!(x: 0.0, y: 0.0, w: 46.0, h: 34.0));
                // self.sprite.collisions_enabled();
                PLAYDATE.sprite.add_sprite(&self.sprite);
                self.anim = Some(Animation::new(images.bird.clone(), [0, 1].as_ref(), 0.1));
                self.anim.as_mut().unwrap().set_scale(0.5);
            }
            _ => {
                let original_image = match self.kind {
                    ObstacleKind::CactusSmall1 => &images.cactus_small1,
                    ObstacleKind::CactusSmall2 => &images.cactus_small2,
                    ObstacleKind::CactusSmall3 => &images.cactus_small3,
                    ObstacleKind::CactusBig1 => &images.cactus_big1,
                    ObstacleKind::CactusBig2 => &images.cactus_big2,
                    ObstacleKind::CactusBig3 => &images.cactus_big3,
                    _ => unreachable!(),
                };
                let size = original_image.get_bitmap_data().size;
                let image: Bitmap = Bitmap::new(
                    size.width as u32 / 2,
                    size.height as u32 / 2,
                    LCDSolidColor::kColorClear,
                );
                PLAYDATE.graphics.push_context(&image);
                PLAYDATE
                    .graphics
                    .draw_scaled_bitmap(&original_image, vec2!(0, 0), vec2!(0.5, 0.5));
                PLAYDATE.graphics.pop_context();
                self.sprite
                    .set_image(image, LCDBitmapFlip::kBitmapUnflipped);
                self.sprite
                    .set_bounds(rect!(x: pos_x, y: DISPLAY_HEIGHT as f32 - Ground::COLLIDE_HEIGHT - size.height as f32 / 2.0, w: size.width as f32 / 2.0, h: size.height as f32 / 2.0));
                // self.sprite.set_collide_rect(
                //     rect!(x: 0.0, y: 0.0, w: size.width as f32 / 2.0, h: size.height as f32 / 2.0),
                // );
                // self.sprite.collisions_enabled();
                PLAYDATE.sprite.add_sprite(&self.sprite);
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        if *DinoGame::get().state.borrow() != GameState::Playing {
            return;
        }
        let pos = self.sprite.get_position();
        self.sprite
            .move_to(vec2!(x: pos.x - 100.0 * delta, y: pos.y));
        let rect = self.sprite.get_bounds();
        let dino_rect = DinoGame::get().dino.get_collide_rect();
        println!("update2 {:?} {:?}", rect, dino_rect);
        if rect.intersects(&dino_rect) {
            println!("Collision!");
            *DinoGame::get().state.borrow_mut() = GameState::Dead;
        }
        if let Some(anim) = &self.anim {
            PLAYDATE
                .graphics
                .push_context(self.sprite.get_image().unwrap());
            PLAYDATE.graphics.clear(LCDSolidColor::kColorBlack);
            anim.play(delta);
            PLAYDATE.graphics.pop_context();
        }
    }
}

pub struct Obstacles {
    images: ObstacleImages,
    obstacles: Vec<Obstacle>,
}

impl Obstacles {
    pub fn new() -> Self {
        Self {
            images: ObstacleImages {
                bird: Arc::new(BitmapTable::open(2, 92, 68, "bird").unwrap()),
                cactus_small1: Bitmap::open(34, 70, "cactus/cactus-small-1").unwrap(),
                cactus_small2: Bitmap::open(68, 70, "cactus/cactus-small-2").unwrap(),
                cactus_small3: Bitmap::open(102, 70, "cactus/cactus-small-3").unwrap(),
                cactus_big1: Bitmap::open(50, 100, "cactus/cactus-big-1").unwrap(),
                cactus_big2: Bitmap::open(100, 100, "cactus/cactus-big-2").unwrap(),
                cactus_big3: Bitmap::open(150, 100, "cactus/cactus-big-3").unwrap(),
            },
            obstacles: vec![],
        }
    }

    pub fn update(&mut self, delta: f32) {
        // Update obstacles
        for obstacle in &mut self.obstacles {
            obstacle.update(delta);
        }
        // remove obstacles that are off screen
        self.obstacles.retain(|obstacle| {
            let rect = obstacle.sprite.get_bounds();
            // println!("update3 {:?}", rect);
            rect.x + rect.width >= 0.0
        });
        // Add new obstacles
        if self.obstacles.is_empty() {
            self.obstacles.push(Obstacle::new(&self.images));
        }
    }
}
