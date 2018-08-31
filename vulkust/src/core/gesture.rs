use super::types::Real;
use super::event::{Event, Type as EventType, Touch, TouchAction, TouchGesture, Move};

use std::time::Instant;
use std::collections::BTreeMap;

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
    pub number: i8,
    pub index: i8,
    pub current: (Real, Real),
    pub start: (Real, Real),
    pub start_time: Instant,
}

impl TouchDragTranslator {
    pub fn new() -> Self {
        TouchDragTranslator {
            state: State::Ended,
            number: 0,
            index: -1,
            current: (0.0, 0.0),
            start: (0.0, 0.0),
            start_time: Instant::now(),
        }
    }

    pub fn receive(&mut self, e: &Event) -> Option<Event> {
        match &e.event_type {
            EventType::Touch(t) => match t {
                Touch::Raw { index: _, action, point: _ } => match action {
                    TouchAction::Press => {
                        self.number += 1;
                    },
                    TouchAction::Release => {
                        self.number -= 1;
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
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
                                    self.index = -1;
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
                                    self.index = -1;
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
            State::Ended => match &e.event_type {
                EventType::Touch(t) => match t {
                    Touch::Raw { index, action, point } => match action {
                        TouchAction::Press => {
                            if self.number == 1 {
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
                                self.index = *index;
                                self.state = State::Started;
                                self.current = *point;
                                self.start = *point;
                                self.start_time = Instant::now();
                                return Some(result);
                            }
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            State::Canceled => vxunexpected!(),
        }
        return None;
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TouchScaleTranslator {
    pub state: State,
    pub fingers: BTreeMap<i8, (Real, Real)>,
    pub first: i8,
    pub second: i8,
    pub current: Real,
    pub start: Real,
    pub start_time: Instant,
}

impl TouchScaleTranslator {
    pub fn new() -> Self {
        TouchScaleTranslator {
            state: State::Ended,
            fingers: BTreeMap::new(),
            first: -1,
            second: -1,
            current: 0.0,
            start: 0.0,
            start_time: Instant::now(),
        }
    }

    pub fn receive(&mut self, e: &Event) -> Option<Event> {
        match &e.event_type {
            EventType::Touch(t) => match t {
                Touch::Raw { index, action, point } => match action {
                    TouchAction::Press => {
                        self.fingers.insert(*index, *point);
                    },
                    TouchAction::Release => {
                        self.fingers.remove(index);
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
        let state = self.state.clone();
        match state {
            State::Started => {
                if self.fingers.len() != 2 {
                    let result = Event::new(EventType::Touch(Touch::Gesture {
                        start_time: self.start_time,
                        duration: Instant::now().duration_since(self.start_time),
                        state: State::Ended,
                        gest: TouchGesture::Scale {
                            first: (-1, (0.0, 0.0)),
                            second: (-1, (0.0, 0.0)),
                            start: self.start,
                            previous: self.current,
                            current: self.current,
                            delta: 0.0,
                        },
                    }));
                    self.first = -1;
                    self.second = -1;
                    self.state = State::Ended;
                    self.current = 0.0;
                    self.start = 0.0;
                    return Some(result);
                }
                match &e.event_type {
                    EventType::Move(m) => match m {
                        Move::Touch {index, previous: _, current, delta: _} => {
                            self.fingers.insert(*index, *current);
                            let f1 = vxunwrap!(self.fingers.get(&self.first));
                            let f2 = vxunwrap!(self.fingers.get(&self.second));
                            let csx = f1.0 - f2.0;
                            let csy = f1.1 - f2.1;
                            let cs = (csx * csx + csy * csy).sqrt();
                            if cs != self.current {
                                let result = Event::new(EventType::Touch(Touch::Gesture {
                                    start_time: self.start_time,
                                    duration: Instant::now().duration_since(self.start_time),
                                    state: State::InMiddle,
                                    gest: TouchGesture::Scale {
                                        first: (self.first, *f1),
                                        second: (self.second, *f2),
                                        start: self.start,
                                        previous: self.current,
                                        current: cs,
                                        delta: cs - self.current,
                                    },
                                }));
                                self.current = cs;
                                return Some(result);
                            }
                        },
                        _ => (),
                    },
                    _ => (),
                }
            },
            State::Ended => {
                if self.fingers.len() == 2 {
                    let mut fi = 0;
                    for f in &self.fingers {
                        if fi == 0 {
                            self.first = *f.0;
                        } else {
                            self.second = *f.0;
                        }
                        fi += 1;
                    }
                    let f1 = vxunwrap!(self.fingers.get(&self.first));
                    let f2 = vxunwrap!(self.fingers.get(&self.second));
                    let csx = f1.0 - f2.0;
                    let csy = f1.1 - f2.1;
                    self.current = (csx * csx + csy * csy).sqrt();
                    self.start = self.current;
                    self.start_time = Instant::now();
                    return Some(Event::new(EventType::Touch(Touch::Gesture {
                        start_time: self.start_time,
                        duration: Instant::now().duration_since(self.start_time),
                        state: State::Started,
                        gest: TouchGesture::Scale {
                            first: (self.first, *f1),
                            second: (self.second, *f2),
                            start: self.start,
                            previous: self.current,
                            current: self.current,
                            delta: 0.0,
                        },
                    })));
                }
            },
            _ => vxunexpected!(),
        }
        return None;
    }
}