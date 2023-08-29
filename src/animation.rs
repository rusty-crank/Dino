use core::cell::RefCell;

use alloc::{borrow::ToOwned, collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use playdate_rs::{
    graphics::{BitmapTable, LCDSolidColor},
    sprite::Sprite,
    PLAYDATE,
};

pub struct Animation {
    table: Arc<BitmapTable>,
    frames: Vec<usize>,
    frame_time: f32,
    current_frame: RefCell<usize>,
    current_time: RefCell<f32>,
    scale: f32,
}

impl Animation {
    pub fn new(table: Arc<BitmapTable>, frames: impl AsRef<[usize]>, frame_time: f32) -> Self {
        Self {
            table,
            frames: frames.as_ref().to_vec(),
            frame_time,
            current_frame: RefCell::new(0),
            current_time: RefCell::new(0.0),
            scale: 1.0,
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn reset(&self) {
        *self.current_frame.borrow_mut() = 0;
        *self.current_time.borrow_mut() = 0.0;
    }

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
    }

    fn draw(&self) {
        PLAYDATE.graphics.draw_scaled_bitmap(
            self.table
                .get(self.frames[*self.current_frame.borrow()])
                .unwrap(),
            vec2!(0, 0),
            vec2!(self.scale, self.scale),
        );
    }

    pub fn play(&self, delta: f32) {
        self.update(delta);
        self.draw();
    }
}

pub trait AnimationState: PartialEq + Clone + Ord {
    const INITIAL: Self;
    type Payload;
    fn transition(&self, sprite: &Self::Payload, delta: f32) -> Option<Self>;
}

pub struct AnimationStateMachine<S: AnimationState> {
    bitmap_tables: BTreeMap<String, Arc<BitmapTable>>,
    animations: BTreeMap<S, Animation>,
    current_state: RefCell<S>,
    scale: f32,
}

impl<S: AnimationState> AnimationStateMachine<S> {
    pub fn new() -> Self {
        Self {
            bitmap_tables: BTreeMap::new(),
            animations: BTreeMap::new(),
            current_state: RefCell::new(S::INITIAL),
            scale: 1.0,
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        for (_, animation) in self.animations.iter_mut() {
            animation.set_scale(scale);
        }
    }

    pub fn add_bitmap_table(
        &mut self,
        name: impl AsRef<str>,
        path: impl AsRef<str>,
        count: usize,
        width: u32,
        height: u32,
    ) {
        let table = Arc::new(BitmapTable::open(count, width, height, path).unwrap());
        self.bitmap_tables
            .insert(name.as_ref().to_owned(), table.clone());
    }

    pub fn add_state(
        &mut self,
        state: S,
        bitmap: impl AsRef<str>,
        frames: impl AsRef<[usize]>,
        frame_time: f32,
    ) {
        let table = &self.bitmap_tables[bitmap.as_ref()];
        let mut animation = Animation::new(table.clone(), frames, frame_time);
        animation.set_scale(self.scale);
        self.animations.insert(state, animation);
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
        PLAYDATE.graphics.clear(LCDSolidColor::kColorClear);
        animation.play(delta);
        PLAYDATE.graphics.pop_context();
    }

    pub fn get_current_state(&self) -> S {
        self.current_state.borrow().clone()
    }
}
