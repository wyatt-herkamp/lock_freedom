#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate fuzzsuite;
extern crate lockfree;

use fuzzsuite::*;
use lockfree::darc::Darc;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
struct DarcMachine {
    ptr: Arc<Darc<u8>>,
    cache: Arc<u8>,
}

impl Spawn for DarcMachine {
    fn spawn() -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}

impl Machine for DarcMachine {
    #[allow(unused_must_use)]
    fn interpret(&mut self, byte: u8, _bytecode: &mut Bytecode) {
        match byte % 5 {
            0 => {
                self.cache = self.ptr.load();
            },

            1 => {
                self.ptr.store(self.cache.clone());
            },

            2 => {
                let init = self.ptr.load();
                self.ptr.compare_and_swap(init, self.cache.clone());
            },

            3 => {
                let init = self.ptr.load();
                self.ptr.compare_exchange(init, self.cache.clone());
            },

            4 => {
                let init = self.ptr.load();
                self.ptr.compare_exchange_weak(init, self.cache.clone());
            },

            _ => unreachable!(),
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let _ = test::<DarcMachine>(Bytecode::no_symbols(data));
});
