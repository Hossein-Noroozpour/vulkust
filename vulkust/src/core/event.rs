use super::gesture::State as GestureState;
use super::object::{create_id, Object};
use super::types::{Id, Real};
use std::time::{Duration, Instant};

pub type FingerIndexType = i64;

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Mouse {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Offic,
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Keyboard {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape(u8),
    Function(u8),
    PrintScreen,
    ScrollLock,
    PauseBreak,
    BackQuote,
    Number { number: u8, padd: bool },
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    NumLock,
    Slash(u8),
    Star,
    Plus(u8),
    Minus(u8),
    Enter(u8),
    Period(u8),
    Tab,
    SquareBracketLeft,
    SquareBracketRight,
    CapseLock,
    SemiColon,
    Quotem,
    BackSlash(u8),
    Shift(u8),
    Comma,
    Control(u8),
    Alt(u8),
    Space(u8),
    Command(u8),
    Super(u8),
    Properties(u8),
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Equal,
    Unknown,
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Button {
    Mouse(Mouse),
    Keyboard(Keyboard),
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Window {
    SizeChange {
        w: Real,
        h: Real,
        ratio: Real,
        pre_w: Real,
        pre_h: Real,
        pre_ratio: Real,
    },
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Move {
    Mouse {
        previous: (Real, Real),
        current: (Real, Real),
        delta: (Real, Real),
    },
    Touch {
        index: FingerIndexType,
        previous: (Real, Real),
        current: (Real, Real),
        delta: (Real, Real),
    },
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum ButtonAction {
    Press,
    Release,
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TouchAction {
    Press,
    HardPress,
    Release,
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TouchGesture {
    Tap, // todo
    Drag {
        index: FingerIndexType,
        start: (Real, Real),
        previous: (Real, Real),
        current: (Real, Real),
        delta: (Real, Real),
    },
    Scale {
        first: (FingerIndexType, (Real, Real)),
        second: (FingerIndexType, (Real, Real)),
        start: Real,
        previous: Real,
        current: Real,
        delta: Real,
    },
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Touch {
    Gesture {
        start_time: Instant,
        duration: Duration,
        state: GestureState,
        gest: TouchGesture,
    },
    Raw {
        index: FingerIndexType,
        action: TouchAction,
        point: (Real, Real),
    },
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Type {
    Move(Move),
    Button {
        button: Button,
        action: ButtonAction,
    },
    Touch(Touch),
    Window(Window),
    Quit,
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Event {
    id: Id,
    pub event_type: Type,
}

impl Event {
    pub fn new(event_type: Type) -> Self {
        Event {
            id: create_id(),
            event_type,
        }
    }
}

impl Object for Event {
    fn get_id(&self) -> Id {
        self.id
    }
}
