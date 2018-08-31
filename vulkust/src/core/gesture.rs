use super::types::Real;
use super::event::{Event, Type as EventType, Touch, TouchAction, TouchGesture, Move};

use std::time::Instant;

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
        if let Some(e) = self.drag.receive(e) {
            v.push(e);
        }
        if let Some(e) = self.scale.receive(e) {
            v.push(e);
        }
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
    pub start_time: Instant,
}

impl TouchDragTranslator {
    pub fn new() -> Self {
        TouchDragTranslator {
            state: State::Ended,
            number: 0,
            index: 0,
            current: (0.0, 0.0),
            start: (0.0, 0.0),
            start_time: Instant::now(),
        }
    }

    pub fn receive(&mut self, e: &Event) -> Option<Event> {
        let state = self.state.clone();
        match state {
            State::Started => match &e.event_type {
                EventType::Touch(t) => match t {
                    Touch::Raw { index, action, point } => {
                        if *index == self.index {
                            match action {
                                TouchAction::Release => {
                                    let result = Event::new(EventType::Touch(Touch::Gesture {
                                        start_time: self.start_time,
                                        duration: Instant::now().duration_since(self.start_time),
                                        state: State::Ended,
                                        gest: TouchGesture::Drag {
                                            index: self.index,
                                            start: self.start,
                                            previous: self.current,
                                            current: *point,
                                            delta: (point.0 - self.current.0, point.1 - self.current.1),
                                        },
                                    }));
                                    self.index = 0;
                                    self.number -= 1;
                                    self.state = State::Ended;
                                    self.current = (0.0, 0.0);
                                    self.start = (0.0, 0.0);
                                    return Some(result);
                                },
                                _ => (),
                            }
                        } else {
                            match action {
                                TouchAction::Press | TouchAction::HardPress => {
                                    let result = Event::new(EventType::Touch(Touch::Gesture {
                                        start_time: self.start_time,
                                        duration: Instant::now().duration_since(self.start_time),
                                        state: State::Canceled,
                                        gest: TouchGesture::Drag {
                                            index: self.index,
                                            start: self.start,
                                            previous: self.current,
                                            current: self.current,
                                            delta: (0.0, 0.0),
                                        },
                                    }));
                                    self.index = 0;
                                    self.number += 1;
                                    self.state = State::Ended;
                                    self.current = (0.0, 0.0);
                                    self.start = (0.0, 0.0);
                                    return Some(result);
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                },
                EventType::Move(m) => match m {
                    Move::Touch { index, previous, current, delta } => {
                        if *index != self.index {
                            vxunexpected!();
                        }
                        self.current = *current;
                        return Some(Event::new(EventType::Touch(Touch::Gesture {
                            start_time: self.start_time,
                            duration: Instant::now().duration_since(self.start_time),
                            state: State::InMiddle,
                            gest: TouchGesture::Drag {
                                index: self.index,
                                start: self.start,
                                previous: *previous,
                                current: *current,
                                delta: *delta,
                            },
                        })));
                    },
                    _ => (),
                },
                _ => (),
            },
            State::InMiddle => vxunexpected!(),
            State::Ended => (),
            State::Canceled => vxunexpected!(),
        }
        return None;
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

    pub fn receive(&mut self, e: &Event) -> Option<Event> {
        vxunimplemented!();
    }
}