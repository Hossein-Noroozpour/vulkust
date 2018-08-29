use super::types::Real;
use super::event::{Event, Type as EventType};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Translator {
    pub drag: TouchDragTranslator,
    pub scale: TouchScaleTranslator,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            drag: TouchDragTranslator::new(),
            scale: TouchScaleTranslator::new(),
        }
    }

    pub fn receive(&mut self, e: &Event) -> Vec<Event> {
        let mut v = Vec::new();
        v.push(self.drag.receive(e));
        v.push(self.scale.receive(e));
        return v;
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum State {
    Started,
    InMiddle,
    Ended,
    Canceled,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TouchDragTranslator {
    pub state: State,
    pub number: u8,
    pub index: u8,
    pub current: (Real, Real),
    pub start: (Real, Real),
}

impl TouchDragTranslator {
    pub fn new() -> Self {
        TouchDragTranslator {
            state: State::Canceled,
            number: 0,
            index: 0,
            current: (0.0, 0.0),
            start: (0.0, 0.0),
        }
    }

    pub fn receive(&mut self, e: &Event) -> Event {
        vxunimplemented!();
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TouchScaleTranslator {
    pub state: State,
    pub number: u8,
    pub index: u8,
    pub current: Real,
    pub start: Real,
}

impl TouchScaleTranslator {
    pub fn new() -> Self {
        TouchScaleTranslator {
            state: State::Canceled,
            number: 0,
            index: 0,
            current: 0.0,
            start: 0.0,
        }
    }

    pub fn receive(&mut self, e: &Event) -> Event {
        vxunimplemented!();
    }
}