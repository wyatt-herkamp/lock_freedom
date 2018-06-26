#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate lockfree;

use lockfree::prelude::*;
use std::{sync::Arc, thread, time::Duration};

mod global {
    // Any combination of these operations should not panic/SIGSEGV.
    pub const NEW_THREAD: u8 = 3;
    pub const AWAIT_LAST_THREAD: u8 = 4;
    pub const SEND_NEXT: u8 = 0;
    pub const RECV_AND_SEND: u8 = 1;
    pub const SEND_NEXT_2: u8 = 2;
}

mod local {
    // Any combination of these operations should not panic/SIGSEGV.
    pub const SEND_NEXT: u8 = 0;
    pub const RECV_TIMEOUT_NEXT_AND_SEND: u8 = 1;
    pub const RECV: u8 = 2;
    pub const RECV_AND_SEND: u8 = 3;
}

#[derive(Debug, Clone)]
struct CommonState {
    sender: Sender<Box<u8>>,
    receiver: Receiver<Box<u8>>,
    code: Arc<[u8]>,
}

impl CommonState {
    fn new(code: &[u8]) -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver,
            code: code.into(),
        }
    }
}

#[derive(Debug)]
struct LocalState {
    index: usize,
    common: CommonState,
}

impl LocalState {
    fn new(common: CommonState) -> Self {
        Self {
            index: 0,
            common,
        }
    }

    fn eval(&mut self, op: u8) {
        loop {
            match op % 3 {
                local::SEND_NEXT => {
                    if let Some(&b) = self.common.code.get(self.index) {
                        self.index = self.index.wrapping_add(1);
                        self.common.sender.send(Box::new(b));
                    }
                },

                local::RECV_TIMEOUT_NEXT_AND_SEND => {
                    if let Some(&b) = self.common.code.get(self.index) {
                        if let Some(x) = self
                            .common
                            .receiver
                            .recv_timeout(Duration::new(1, b as u32 * 100))
                        {
                            self.common.sender.send(x);
                        }
                    }
                },

                local::RECV => {
                    self.common.receiver.recv();
                },

                local::RECV_AND_SEND => {
                    if let Some(val) = self.common.receiver.recv() {
                        self.common.sender.send(val);
                    }
                },

                _ => (),
            }
            break;
        }
    }

    fn run(mut self) {
        while let Some(&op) = self.common.code.get(self.index) {
            self.index = self.index.wrapping_add(1);
            self.eval(op);
        }
    }
}

#[derive(Debug)]
struct GlobalState {
    index: usize,
    common: CommonState,
    threads: Vec<thread::JoinHandle<()>>,
}

impl GlobalState {
    fn new(common: CommonState) -> Self {
        Self {
            common,
            index: 0,
            threads: Vec::new(),
        }
    }

    fn eval(&mut self, op: u8) {
        match op % 5 {
            global::NEW_THREAD => {
                let local = LocalState::new(self.common.clone());
                self.threads.push(thread::spawn(move || local.run()));
            },

            global::AWAIT_LAST_THREAD => {
                if let Some(thread) = self.threads.pop() {
                    thread.join().unwrap();
                }
            },

            global::SEND_NEXT | global::SEND_NEXT_2 => {
                if let Some(&b) = self.common.code.get(self.index) {
                    self.index = self.index.wrapping_add(1);
                    self.common.sender.send(Box::new(b));
                }
            },

            global::RECV_AND_SEND => {
                if let Some(val) = self.common.receiver.recv() {
                    self.common.sender.send(val);
                }
            },

            _ => (),
        }
    }

    fn run(mut self) {
        while let Some(&op) = self.common.code.get(self.index) {
            self.index = self.index.wrapping_add(1);
            self.eval(op);
        }
        for thread in self.threads {
            thread.join().unwrap();
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let global = GlobalState::new(CommonState::new(data));
    global.run();
});
