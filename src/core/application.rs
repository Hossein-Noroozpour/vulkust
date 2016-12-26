use std::thread::{
    JoinHandle,
    spawn,
};

use super::super::system::application::Application as SysApp;

pub struct BasicApplication<SA> where SA: SysApp {
    pub thread: JoinHandle<()>,
    pub sys_app: SA,
}

pub trait Application {
}

impl<SA> BasicApplication<SA> where SA: SysApp {
    pub fn new(sys_app: SA) -> Self {
        BasicApplication {
            thread: BasicApplication::<SA>::make_thread(),
            sys_app: sys_app,
        }
    }

    fn main() {
//        loop {
//            logdbg!("In main thread.");
//        }
    }

    fn make_thread() -> JoinHandle<()> {
        spawn(|| {
            BasicApplication::<SA>::main();
        })
    }
}