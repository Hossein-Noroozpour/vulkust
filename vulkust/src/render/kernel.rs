use super::engine::Engine;
use std::sync::mpsc::{channel, Sender};
use std::sync::{RwLock, Weak};
use std::thread::{spawn, JoinHandle};

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Kernel {
    loop_signaler: Sender<bool>,
    handle: JoinHandle<()>,
}

impl Kernel {
    pub fn new(index: usize, engine: Weak<RwLock<Engine>>) -> Self {
        let (loop_signaler, rcv) = channel();
        let handle = spawn(move || {
            let mut renderer = Renderer::new(index, engine);
            while vxresult!(rcv.recv()) {
                renderer.render();
            }
        });
        Self {
            loop_signaler,
            handle,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct Renderer {
    index: usize,
    engine: Weak<RwLock<Engine>>,
}

impl Renderer {
    pub fn new(index: usize, engine: Weak<RwLock<Engine>>) -> Self {
        Renderer { index, engine }
    }

    pub fn render(&mut self) {}
}
