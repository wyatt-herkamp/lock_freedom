#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate fuzzsuite;
extern crate lockfree;

use fuzzsuite::*;
use lockfree::{map::Preview, prelude::*};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BadHash(u128);

impl Hash for BadHash {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write_u8(self.0 as u8);
        hasher.write_u8((self.0 >> 16) as u8);
        hasher.write_u8((self.0 >> 32) as u8);
        hasher.write_u8((self.0 >> 48) as u8);
        hasher.write_u8((self.0 >> 64) as u8);
        hasher.write_u8((self.0 >> 80) as u8);
        hasher.write_u8((self.0 >> 96) as u8);
        hasher.write_u8((self.0 >> 112) as u8);
    }
}

impl BadHash {
    fn from_symbol(sym: &[u8], decision: u8) -> Self {
        let mut i = 0xA91C;
        for &byte in sym {
            i = (byte as u128)
                .wrapping_mul(i ^ i >> 16 ^ byte as u128 >> 2)
                .wrapping_mul((decision ^ byte) as u128)
                .wrapping_mul(decision as u128 ^ i)
                .wrapping_mul(decision as u128 ^ i)
                .wrapping_add((decision ^ byte) as u128)
        }
        BadHash(i)
    }
}

#[derive(Debug, Clone, Default)]
struct MapMachine {
    map: Arc<Map<BadHash, u8>>,
    key: u8,
    val: u8,
    decision: u8,
}

impl Machine for MapMachine {
    fn spawn() -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        self.clone()
    }

    fn interpret(&mut self, byte: u8, bytecode: &mut Bytecode) {
        match byte % 10 {
            0 => {
                self.key = bytecode.next().unwrap_or(0);
                self.val = bytecode.next().unwrap_or(0);
                self.decision = bytecode.next().unwrap_or(0);
            },

            1 | 5 => {
                let key = BadHash::from_symbol(
                    bytecode.symbol(self.key),
                    self.decision,
                );
                self.map.insert(key, self.val);
            },

            2 => {
                self.key = bytecode.next().unwrap_or(0);
                let key = BadHash::from_symbol(
                    bytecode.symbol(self.key),
                    self.decision,
                );
                self.val = self.map.get(&key, |&byte| byte).unwrap_or(0);
            },

            3 => {
                let key = BadHash::from_symbol(
                    bytecode.symbol(self.key),
                    self.decision,
                );;
                self.val = self.map.get(&key, |&byte| byte).unwrap_or(0);
                self.key = bytecode.next().unwrap_or(0);
                self.decision ^= self.key;
            },

            4 => {
                let key = BadHash::from_symbol(
                    bytecode.symbol(self.key),
                    self.decision,
                );;
                self.val = self.map.get(&key, |&byte| byte).unwrap_or(0);
                self.key = bytecode.next().unwrap_or(0);
                self.decision ^= self.val;
            },

            6 => {
                self.key = self.val;
            },

            7 => {
                let key = BadHash::from_symbol(
                    bytecode.symbol(self.key),
                    self.decision,
                );
                let decision = bytecode.next().unwrap_or(0);
                let inc = bytecode.next().unwrap_or(0);
                self.map.insert_with(key, |key, stored, prev| {
                    match decision % 8 {
                        0 | 1 => Preview::Discard,
                        2 | 3 => Preview::Keep,
                        4 => Preview::New(
                            inc.wrapping_add(key.0 as u8)
                                .wrapping_add(stored.map_or(0, |&x| x))
                                .wrapping_add(prev.map_or(0, |&x| x)),
                        ),
                        5 => Preview::New(inc.wrapping_add(key.0 as u8)),
                        6 => Preview::New(
                            inc.wrapping_add(stored.map_or(0, |&x| x)),
                        ),
                        7 => Preview::New(
                            (key.0 as u8)
                                .wrapping_add(stored.map_or(0, |&x| x)),
                        ),
                        _ => unreachable!(),
                    }
                });
            },

            8 => {
                let key = BadHash::from_symbol(
                    bytecode.symbol(self.key),
                    self.decision,
                );;
                let removed = match self.map.remove(&key) {
                    Some(x) => x,
                    None => return (),
                };
                let decision = bytecode.next().unwrap_or(0);
                let test = bytecode.next().unwrap_or(0);
                self.map.reinsert_with(
                    removed,
                    |removed, stored| match decision % 5 {
                        0 => removed.val().wrapping_add(test) % 2 == 0,
                        1 => removed.val().wrapping_mul(test) % 2 == 0,
                        2 => {
                            let res = removed
                                .val()
                                .wrapping_mul(test ^ stored.map_or(0, |&x| x));
                            res % 2 == 0
                        },
                        3 => stored.is_some(),
                        4 => stored.is_none(),
                        _ => unreachable!(),
                    },
                );
            },

            9 => {
                let mut sum = 0u128;
                for vec in &*self.map {
                    for (&k, &v) in vec {
                        sum += k.0.wrapping_add(v as u128);
                    }
                }
                self.key = sum as u8;
            },

            _ => unreachable!(),
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let _ = test::<MapMachine>(Bytecode::new(data));
});
