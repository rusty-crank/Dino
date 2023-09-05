use core::cell::RefCell;

use playdate_rs::{
    display::DISPLAY_HEIGHT,
    graphics::{Bitmap, LCDBitmapFlip, LCDSolidColor},
    math::{Rect, Size, Vec2},
    sprite::Sprite,
    system::Buttons,
    App, PLAYDATE,
};

use crate::{
    animation::{AnimationState, AnimationStateMachine, BitmapAnimation},
    ground::Ground,
    DinoGame, GameState,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum DinoState {
    Idle,
    Run,
    Jump,
    Duck,
    Dead,
}

const IMAGE_SIZE: Size<f32> = size!(160.0, 94.0);
const SPRITE_SIZE: Size<f32> = size!(IMAGE_SIZE.width / 2.0, IMAGE_SIZE.height / 2.0);
const INITLAL_BOUNDS: Rect<f32> = Rect {
    x: 0.0,
    y: DISPLAY_HEIGHT as f32 - Ground::COLLIDE_HEIGHT - SPRITE_SIZE.height,
    width: SPRITE_SIZE.width,
    height: SPRITE_SIZE.height,
};
const INITLAL_POSITION: Vec2<f32> = vec2!(
    INITLAL_BOUNDS.x + INITLAL_BOUNDS.width / 2.0,
    INITLAL_BOUNDS.y + INITLAL_BOUNDS.height / 2.0
);

const COLLIDE_RECT: Rect<f32> = Rect {
    x: (IMAGE_SIZE.width - 88.0) / 2.0 / 2.0,
    y: 0.0,
    width: 88.0 / 2.0,
    height: IMAGE_SIZE.height / 2.0,
};
const DUCK_COLLIDE_RECT: Rect<f32> = Rect {
    x: (IMAGE_SIZE.width - 118.0) / 2.0 / 2.0,
    y: (IMAGE_SIZE.height - 60.0) / 2.0,
    width: 118.0 / 2.0,
    height: 60.0 / 2.0,
};

impl AnimationState for DinoState {
    const INITIAL: Self = Self::Idle;

    type Payload = Dino;

    fn transition(&self, dino: &Dino, _delta: f32) -> Option<Self> {
        let button_state = PLAYDATE.system.get_button_state();
        let bounds = dino.sprite.get_bounds();
        // Idle -> Run
        if self == &Self::Idle {
            if *DinoGame::get().state.borrow() != GameState::Ready {
                return Some(Self::Run);
            }
            return None;
        }
        // Run -> {Jump, Duck, Dead}
        if self == &Self::Run {
            if *DinoGame::get().state.borrow() == GameState::Dead {
                return Some(Self::Dead);
            }
            if button_state.pushed.contains(Buttons::A) {
                return Some(Self::Jump);
            }
            if button_state.current.contains(Buttons::B) {
                return Some(Self::Duck);
            }
            return None;
        }
        // Duck -> {Run, Dead}
        if self == &Self::Duck {
            if *DinoGame::get().state.borrow() == GameState::Dead {
                return Some(Self::Dead);
            }
            if !button_state.current.contains(Buttons::B) {
                return Some(Self::Run);
            }
            return None;
        }
        // Jump -> {Run, Dead}
        if self == &Self::Jump {
            if *DinoGame::get().state.borrow() == GameState::Dead {
                return Some(Self::Dead);
            }
            let bottom = DISPLAY_HEIGHT as f32 - bounds.height - bounds.y;
            if bottom <= Ground::COLLIDE_HEIGHT {
                return Some(Self::Run);
            }
            return None;
        }
        // Dead -> Run
        if self == &Self::Dead {
            if *DinoGame::get().state.borrow() == GameState::Playing {
                return Some(Self::Run);
            }
            return None;
        }
        unreachable!()
    }
}

pub struct Dino {
    animations: AnimationStateMachine<DinoState>,
    sprite: Sprite,
    vertical_velocity: RefCell<f32>,
}

impl Dino {
    pub fn new() -> Self {
        let sprite = Sprite::new();
        let bitmap = Bitmap::new(
            size!(SPRITE_SIZE.width as _, SPRITE_SIZE.height as _),
            LCDSolidColor::kColorClear,
        );
        sprite.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        sprite.set_bounds(INITLAL_BOUNDS);
        sprite.set_collide_rect(COLLIDE_RECT);
        sprite.collisions_enabled();
        PLAYDATE.sprite.add_sprite(&sprite);
        Self {
            sprite,
            animations: Self::create_animation_state_machine(),
            vertical_velocity: RefCell::new(0.0),
        }
    }

    fn create_animation_state_machine() -> AnimationStateMachine<DinoState> {
        let mut asm = AnimationStateMachine::new();
        let table = asm.add_bitmap_table("dino", "dino", 8, 160, 94);
        let anim = |frames: &[usize], frame_time: f32| {
            BitmapAnimation::new(table.clone(), frames, frame_time, 0.5)
        };
        asm.add_state(DinoState::Idle, anim(&[1, 2], 0.5));
        asm.add_state(DinoState::Jump, anim(&[1], 0.5));
        asm.add_state(DinoState::Run, anim(&[3, 4], 0.2));
        asm.add_state(DinoState::Duck, anim(&[5, 6], 0.2));
        asm.add_state(DinoState::Dead, anim(&[7], 1.0));
        asm
    }

    pub fn reset(&mut self) {
        self.sprite.move_to(INITLAL_POSITION);
        self.animations.reset();
    }

    pub fn update(&mut self, delta: f32) {
        // update animation and state
        let old_state = self.animations.get_current_state();
        self.animations.update(&self.sprite, delta, self);
        let state = self.animations.get_current_state();
        if *DinoGame::get().state.borrow() != GameState::Playing {
            return;
        }
        // update collide rect
        self.sprite.set_collide_rect(match state {
            DinoState::Duck => DUCK_COLLIDE_RECT,
            _ => COLLIDE_RECT,
        });
        // update velocity
        let mut velocity = self.vertical_velocity.borrow_mut();
        match (old_state, state) {
            (DinoState::Idle, DinoState::Run) => *velocity = -250.0,
            (DinoState::Run, DinoState::Jump) => *velocity = -250.0,
            (DinoState::Dead, DinoState::Run) => {
                self.sprite.move_to(INITLAL_POSITION);
                *velocity = 0.0;
            }
            _ => {}
        }
        // 2. add gravity
        *velocity += 500.0 * delta;
        // update position
        let step = *velocity * delta;
        let mut pos = self.sprite.get_position();
        let old_y = pos.y;
        pos.y += step;
        if pos.y > INITLAL_POSITION.y {
            pos.y = INITLAL_POSITION.y;
        }
        self.sprite.move_to(pos);
        let collisions = self.sprite.check_collisions(pos);
        if collisions.len() > 0 {
            if !collisions
                .iter()
                .all(|x| DinoGame::get().ground.sprite_is_ground(&x.other))
            {
                *DinoGame::get().state.borrow_mut() = GameState::Dead;
                return;
            }
        }
        let pos2 = self.sprite.get_position();
        if pos2.y == old_y {
            *velocity = 0.0;
        }
    }
}
