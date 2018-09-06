#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate fuzzsuite;
extern crate lockfree;

use fuzzsuite::*;
use lockfree::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
struct MapMachine {
    map: Arc<Map<Vec<u8>, u8>>,
    key: u8,
    val: u8,
}

impl Machine for MapMachine {
    fn spawn() -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        self.clone()
    }

    fn interpret(&mut self, byte: u8, bytecode: &mut Bytecode) {
        match byte % 7 {
            0 => {
                self.key = bytecode.next().unwrap_or(0);
                self.val = bytecode.next().unwrap_or(0);
            },

            1 | 5 => {
                let key = bytecode.symbol(self.key).into();
                self.map.insert(key, self.val);
            },

            2 => {
                self.key = bytecode.next().unwrap_or(0);
                let key = bytecode.symbol(self.key);
                self.val = self.map.get(key, |&byte| byte).unwrap_or(0);
            },

            3 => {
                self.val = {
                    let key = bytecode.symbol(self.key);
                    self.map.get(key, |&byte| byte).unwrap_or(0)
                };
                self.key = bytecode.next().unwrap_or(0);
            },

            4 => {
                self.val = {
                    let key = bytecode.symbol(self.key);
                    self.map.get(key, |&byte| byte).unwrap_or(0)
                };
                self.key = bytecode.next().unwrap_or(0);
            },

            6 => {
                self.key = self.val;
            },

            _ => unreachable!(),
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let _ = test::<MapMachine>(Bytecode::new(data));
});
