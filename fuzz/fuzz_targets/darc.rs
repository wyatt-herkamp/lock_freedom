#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate lockfree;

use lockfree::darc::Darc;
use std::{sync::Arc, thread};

mod global {
    // Any combination of these operations should not panic/SIGSEGV.
    pub const NEW_THREAD: u8 = 5;
    pub const AWAIT_LAST_THREAD: u8 = 6;
}

mod local {
    // Any combination of these operations should not panic/SIGSEGV.
    pub const LOAD: u8 = 0;
    // As Darc's store is a wrapper over swap, we will use only swap.
    pub const SWAP: u8 = 1;
    pub const CAS: u8 = 2;
    pub const LOOP_CAS: u8 = 3;
    pub const CX: u8 = 4;
    pub const LOOP_CX: u8 = 5;
    pub const CX_WEAK: u8 = 6;
    pub const LOOP_CX_WEAK: u8 = 7;
}

fn local_op(data: Arc<[u8]>, target: Arc<Darc<String>>) {
    let mut idx = 0;
    let mut regs = [Arc::new("abc".into()), Arc::new("abc".into())];
    while let Some(&op) = data.get(idx) {
        macro_rules! get_reg {
            () => {
                match data.get(idx) {
                    Some(&reg) => {
                        idx += 1;
                        (reg % 2) as usize
                    },
                    _ => break,
                }
            };
        }
        idx += 1;
        match op % 8 {
            local::LOAD => {
                let dest = get_reg!();
                regs[dest] = target.load();
            },

            local::SWAP => {
                let src = get_reg!();
                let dest = get_reg!();
                regs[dest] = target.swap(regs[src].clone());
            },

            local::CAS => {
                let cmp = get_reg!();
                let src = get_reg!();
                let dest = get_reg!();
                regs[dest] = target
                    .compare_and_swap(regs[cmp].clone(), regs[src].clone());
            },

            local::LOOP_CAS => {
                let src = get_reg!();
                loop {
                    let initial = target.load();
                    let res = target
                        .compare_and_swap(initial.clone(), regs[src].clone());
                    if Arc::ptr_eq(&initial, &res) {
                        break;
                    }
                }
            },

            local::CX => {
                let cmp = get_reg!();
                let src = get_reg!();
                let dest = get_reg!();
                regs[dest] = match target
                    .compare_exchange(regs[cmp].clone(), regs[src].clone())
                {
                    Ok(x) => x,
                    Err(x) => x,
                };
            },

            local::LOOP_CX => {
                let src = get_reg!();
                let mut initial = target.load();
                loop {
                    match target
                        .compare_exchange(initial.clone(), regs[src].clone())
                    {
                        Ok(_) => break,
                        Err(p) => initial = p,
                    }
                }
            },

            local::CX_WEAK => {
                let cmp = get_reg!();
                let src = get_reg!();
                let dest = get_reg!();
                regs[dest] = match target
                    .compare_exchange_weak(regs[cmp].clone(), regs[src].clone())
                {
                    Ok(x) => x,
                    Err(x) => x,
                };
            },

            local::LOOP_CX_WEAK => {
                let src = get_reg!();
                let mut initial = target.load();
                loop {
                    match target.compare_exchange_weak(
                        initial.clone(),
                        regs[src].clone(),
                    ) {
                        Ok(_) => break,
                        Err(p) => initial = p,
                    }
                }
            },

            _ => (),
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let mut threads = Vec::new();
    let darc = Arc::new(Darc::from("abc".to_owned()));
    let arc_data = <Arc<[u8]> as From<&[u8]>>::from(data);
    for piece in data {
        match piece % 8 {
            global::NEW_THREAD => {
                let data = arc_data.clone();
                let darc = darc.clone();
                threads.push(thread::spawn(move || local_op(data, darc)))
            },
            global::AWAIT_LAST_THREAD => {
                if let Some(thread) = threads.pop() {
                    thread.join().unwrap();
                }
            },
            _ => (),
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }
});
