use crossterm::event::Event;
use std::borrow::Cow;
use xi_rope::Interval;

use crate::BuffrCollection;

// A mode should OWN all data related to it. Hence we bound it by 'static.
pub trait Mode: 'static {
    // TODO: Maybe this should be just String instead.
    fn name(&self) -> Cow<'static, str>;
    fn transition(
        &self,
        event: &Event,
        buffr_collection: &mut BuffrCollection,
        bytes_per_line: usize,
    ) -> Option<ModeTransition>;

    fn takes_input(&self) -> bool {
        true
    }
    fn has_half_cursor(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DirtyBytes {
    ChangeInPlace(Vec<Interval>),
    ChangeLength,
}

pub enum ModeTransition {
    None,
    NewMode(Box<dyn Mode>),
    DirtyBytes(DirtyBytes),
    ModeAndDirtyBytes(Box<dyn Mode>, DirtyBytes),
    ModeAndInfo(Box<dyn Mode>, String),
}

impl ModeTransition {
    pub fn new_mode(mode: impl Mode) -> ModeTransition {
        ModeTransition::NewMode(Box::new(mode))
    }

    pub fn new_mode_and_dirty(mode: impl Mode, dirty: DirtyBytes) -> ModeTransition {
        ModeTransition::ModeAndDirtyBytes(Box::new(mode), dirty)
    }

    pub fn new_mode_and_info(mode: impl Mode, info: String) -> ModeTransition {
        ModeTransition::ModeAndInfo(Box::new(mode), info)
    }
}
