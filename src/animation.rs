use core::cell::RefCell;

use alloc::{
    borrow::ToOwned, boxed::Box, collections::BTreeMap, string::String, sync::Arc, vec::Vec,
};
use playdate_rs::{graphics::BitmapTable, sprite::Sprite, PLAYDATE};

pub trait Animation {
    fn update(&self, delta: f32);
    fn reset(&self);
}

pub struct BitmapAnimation {
    table: Arc<BitmapTable>,
    frames: Vec<usize>,
    frame_time: f32,
    current_frame: RefCell<usize>,
    current_time: RefCell<f32>,
    scale: f32,
}

impl BitmapAnimation {
    pub fn new(
        table: Arc<BitmapTable>,
        frames: impl AsRef<[usize]>,
        frame_time: f32,
        scale: f32,
    ) -> Self {
        Self {
            table,
            frames: frames.as_ref().to_vec(),
            frame_time,
            current_frame: RefCell::new(0),
            current_time: RefCell::new(0.0),
            scale,
        }
    }

    fn reset(&self) {
        *self.current_frame.borrow_mut() = 0;
        *self.current_time.borrow_mut() = 0.0;
    }
}

impl Animation for BitmapAnimation {
    fn update(&self, delta: f32) {
        let mut current_time = self.current_time.borrow_mut();
        let mut current_frame = self.current_frame.borrow_mut();
        *current_time += delta;
        if *current_time >= self.frame_time {
            *current_frame += 1;
            if *current_frame >= self.frames.len() {
                *current_frame = 0;
            }
            *current_time = 0.0;
        }
        PLAYDATE.graphics.draw_scaled_bitmap(
            self.table.get(self.frames[*current_frame]).unwrap(),
            vec2!(0, 0),
            vec2!(self.scale, self.scale),
        );
    }

    fn reset(&self) {
        self.reset();
    }
}

pub trait AnimationState: PartialEq + Clone + Ord {
    const INITIAL: Self;
    type Payload;
    fn transition(&self, sprite: &Self::Payload, delta: f32) -> Option<Self>;
}

pub struct AnimationStateMachine<S: AnimationState> {
    bitmap_tables: BTreeMap<String, Arc<BitmapTable>>,
    animations: BTreeMap<S, Box<dyn Animation>>,
    current_state: RefCell<S>,
}

impl<S: AnimationState> AnimationStateMachine<S> {
    pub fn new() -> Self {
        Self {
            bitmap_tables: BTreeMap::new(),
            animations: BTreeMap::new(),
            current_state: RefCell::new(S::INITIAL),
        }
    }

    pub fn add_bitmap_table(
        &mut self,
        name: impl AsRef<str>,
        path: impl AsRef<str>,
        count: usize,
        width: u32,
        height: u32,
    ) -> Arc<BitmapTable> {
        let table = Arc::new(BitmapTable::open(count, width, height, path).unwrap());
        self.bitmap_tables
            .insert(name.as_ref().to_owned(), table.clone());
        table
    }

    pub fn add_state(&mut self, state: S, anim: impl Animation + 'static) {
        self.animations.insert(state, Box::new(anim));
    }

    pub fn update(&self, sprite: &Sprite, delta: f32, payload: &S::Payload) {
        let bitmap = sprite.get_image().unwrap();
        let mut current_state = self.current_state.borrow_mut();
        let next_state = current_state
            .transition(payload, delta)
            .unwrap_or(current_state.clone());
        let animation = &self.animations[&next_state];
        if next_state != *current_state {
            animation.reset();
            *current_state = next_state;
        }
        PLAYDATE.graphics.push_context(bitmap);
        PLAYDATE.graphics.clear(crate::sprite_bg_color());
        animation.update(delta);
        PLAYDATE.graphics.pop_context();
    }

    pub fn get_current_state(&self) -> S {
        self.current_state.borrow().clone()
    }

    pub fn reset(&self) {
        *self.current_state.borrow_mut() = S::INITIAL;
        for (_, animation) in self.animations.iter() {
            animation.reset();
        }
    }
}
