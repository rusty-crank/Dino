use core::cell::RefCell;

use playdate_rs::{
    display::DISPLAY_HEIGHT,
    graphics::{Bitmap, LCDBitmapFlip, LCDSolidColor},
    math::{Point2D, Rect, Size2D},
    sprite::Sprite,
    system::Buttons,
    PLAYDATE,
};

use crate::animation::{AnimationState, AnimationStateMachine};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DinoState {
    Idle,
    Run,
    Jump,
    Duck,
    Dead,
}

const COLLIDE_RECT: Rect<f32> = Rect {
    origin: Point2D::new((160.0 - 88.0) / 2.0 / 2.0, 0.0),
    size: Size2D::new(88.0 / 2.0, 94.0 / 2.0),
};

const DUCK_COLLIDE_RECT: Rect<f32> = Rect {
    origin: Point2D::new((160.0 - 118.0) / 2.0 / 2.0, (94.0 - 60.0) / 2.0),
    size: Size2D::new(118.0 / 2.0, 60.0 / 2.0),
};

impl AnimationState for DinoState {
    const INITIAL: Self = Self::Idle;

    type Payload = Dino;

    fn transition(&self, dino: &Dino, _delta: f32) -> Option<Self> {
        let button_state = PLAYDATE.system.get_button_state();
        let bounds = dino.sprite.get_bounds();
        // Idle -> Run
        if self == &Self::Idle {
            if button_state.current.contains(Buttons::A) {
                return Some(Self::Run);
            }
            return None;
        }
        // any collision -> Dead

        // Run -> {Jump, Duck}
        if self == &Self::Run {
            if button_state.pushed.contains(Buttons::A) {
                return Some(Self::Jump);
            }
            if button_state.current.contains(Buttons::B) {
                return Some(Self::Duck);
            }
            return None;
        }
        // Duck -> Run
        if self == &Self::Duck {
            if !button_state.current.contains(Buttons::B) {
                return Some(Self::Run);
            }
            return None;
        }
        // Jump -> Run
        if self == &Self::Jump {
            let bottom = DISPLAY_HEIGHT as f32 - bounds.height() - bounds.origin.y;
            if bottom <= 6.0 {
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
        let bitmap = Bitmap::new(80, 47, LCDSolidColor::kColorClear);
        sprite.set_image(bitmap, LCDBitmapFlip::kBitmapUnflipped);
        sprite.set_bounds(Rect {
            origin: Point2D::new(0.0, 180.0),
            size: Size2D::new(80.0, 47.0),
        });
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
        let mut anim = AnimationStateMachine::new();
        anim.add_bitmap_table("dino", "dino", 8, 160, 94);
        anim.add_state(DinoState::Idle, "dino", [1, 2], 0.5);
        anim.add_state(DinoState::Jump, "dino", [1], 0.5);
        anim.add_state(DinoState::Run, "dino", [3, 4], 0.2);
        anim.add_state(DinoState::Duck, "dino", [5, 6], 0.2);
        anim.set_scale(0.5);
        anim
    }

    pub fn update(&mut self, delta: f32) {
        // update animation and state
        let old_state = self.animations.get_current_state();
        self.animations.update(&self.sprite, delta, self);
        let state = self.animations.get_current_state();
        // update collide rect
        self.sprite.set_collide_rect(match state {
            DinoState::Duck => DUCK_COLLIDE_RECT,
            _ => COLLIDE_RECT,
        });
        // update velocity
        let mut velocity = self.vertical_velocity.borrow_mut();
        match (old_state, state) {
            (DinoState::Run, DinoState::Jump) => *velocity = -250.0,
            _ => {}
        }
        // 2. add gravity
        *velocity += 500.0 * delta;
        // update position
        let step = *velocity * delta;
        let mut pos = self.sprite.get_position();
        let old_y = pos.y;
        pos.y += step;
        PLAYDATE.sprite.move_with_collisions(&self.sprite, pos);
        let pos2 = self.sprite.get_position();
        if pos2.y == old_y {
            *velocity = 0.0;
        }
    }
}
