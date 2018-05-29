#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate lockfree;

use lockfree::prelude::*;
use std::{sync::Arc, thread};

mod global {
    // Any combination of these operations should not panic/SIGSEGV.
    pub const NEW_THREAD: u8 = 0;
    pub const AWAIT_LAST_THREAD: u8 = 1;
    pub const QUEUE_PUSH_NEXT: u8 = 2;
    pub const QUEUE_POP: u8 = 3;
    pub const QUEUE_POP_AND_RUN: u8 = 4;
    pub const STACK_PUSH_NEXT: u8 = 5;
    pub const STACK_POP: u8 = 6;
    pub const STACK_POP_AND_RUN: u8 = 7;
}

mod local {
    // Any combination of these operations should not panic/SIGSEGV.
    pub const CONSUME_QUEUE: u8 = 0;
    pub const CONSUME_STACK: u8 = 1;
    pub const QUEUE_PUSH_NEXT: u8 = 2;
    pub const QUEUE_POP: u8 = 3;
    pub const QUEUE_POP_AND_RUN: u8 = 4;
    pub const STACK_PUSH_NEXT: u8 = 5;
    pub const STACK_POP: u8 = 6;
    pub const STACK_POP_AND_RUN: u8 = 7;
}

#[derive(Debug, Clone)]
struct CommonState {
    queue: Arc<Queue<Box<u8>>>,
    stack: Arc<Stack<Box<u8>>>,
    code: Arc<[u8]>,
}

impl CommonState {
    fn new(code: &[u8]) -> Self {
        Self {
            queue: Arc::new(Queue::new()),
            stack: Arc::new(Stack::new()),
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

    fn eval(&mut self, mut op: u8) {
        loop {
            match op % 8 {
                local::CONSUME_QUEUE => for _ in &*self.common.queue {},
                local::CONSUME_STACK => for _ in &*self.common.stack {},
                local::QUEUE_PUSH_NEXT => {
                    if let Some(&b) = self.common.code.get(self.index) {
                        self.index = self.index.wrapping_add(1);
                        self.common.queue.push(Box::new(b));
                    }
                },
                local::QUEUE_POP => {
                    self.common.queue.pop();
                },
                local::QUEUE_POP_AND_RUN => {
                    if let Some(new_op) = self.common.queue.pop() {
                        op = *new_op;
                        continue;
                    }
                },
                local::STACK_PUSH_NEXT => {
                    if let Some(&b) = self.common.code.get(self.index) {
                        self.index = self.index.wrapping_add(1);
                        self.common.stack.push(Box::new(b));
                    }
                },
                local::STACK_POP => {
                    self.common.stack.pop();
                },
                local::STACK_POP_AND_RUN => {
                    if let Some(new_op) = self.common.stack.pop() {
                        op = *new_op;
                        continue;
                    }
                },
                _ => (),
            }
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

    fn eval(&mut self, mut op: u8) {
        loop {
            match op % 8 {
                global::NEW_THREAD => {
                    let local = LocalState::new(self.common.clone());
                    self.threads.push(thread::spawn(move || local.run()));
                },
                global::AWAIT_LAST_THREAD => {
                    if let Some(thread) = self.threads.pop() {
                        thread.join().unwrap();
                    }
                },
                global::QUEUE_PUSH_NEXT => {
                    if let Some(&b) = self.common.code.get(self.index) {
                        self.index = self.index.wrapping_add(1);
                        self.common.queue.push(Box::new(b));
                    }
                },
                global::QUEUE_POP => {
                    self.common.queue.pop();
                },
                global::QUEUE_POP_AND_RUN => {
                    if let Some(new_op) = self.common.queue.pop() {
                        op = *new_op;
                        continue;
                    }
                },
                global::STACK_PUSH_NEXT => {
                    if let Some(&b) = self.common.code.get(self.index) {
                        self.index = self.index.wrapping_add(1);
                        self.common.stack.push(Box::new(b));
                    }
                },
                global::STACK_POP => {
                    self.common.stack.pop();
                },
                global::STACK_POP_AND_RUN => {
                    if let Some(new_op) = self.common.stack.pop() {
                        op = *new_op;
                        continue;
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
        for thread in self.threads {
            thread.join().unwrap();
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let global = GlobalState::new(CommonState::new(data));
    global.run();
});
