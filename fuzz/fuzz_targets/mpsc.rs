#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate fuzzsuite;
extern crate lockfree;

use fuzzsuite::*;
use lockfree::prelude::*;
use std::sync::Arc;


#[derive(Debug, Clone, Default)]
struct QueueMachine {
    queue: Arc<Queue<Box<u8>>>,
}



fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
