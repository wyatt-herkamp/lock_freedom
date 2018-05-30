#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate lockfree;

use lockfree::darc::*;
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

        macro_rules! get_ord {
            (r) => {
                match data.get(idx) {
                    Some(&byte) => {
                        idx += 1;
                        get_read_ord(byte)
                    },
                    _ => break,
                }
            };
            (rw) => {
                match data.get(idx) {
                    Some(&byte) => {
                        idx += 1;
                        get_rw_ord(byte)
                    },
                    _ => break,
                }
            };
        }

        idx += 1;
        match op % 8 {
            local::LOAD => {
                let dest = get_reg!();
                let ord = get_ord!(r);
                regs[dest] = target.load(ord);
            },

            local::SWAP => {
                let src = get_reg!();
                let dest = get_reg!();
                let ord = get_ord!(rw);
                regs[dest] = target.swap(regs[src].clone(), ord);
            },

            local::CAS => {
                let cmp = get_reg!();
                let src = get_reg!();
                let dest = get_reg!();
                let ord = get_ord!(rw);
                regs[dest] = target.compare_and_swap(
                    regs[cmp].clone(),
                    regs[src].clone(),
                    ord,
                );
            },

            local::LOOP_CAS => {
                let src = get_reg!();
                let load_ord = get_ord!(r);
                let cas_ord = get_ord!(rw);
                loop {
                    let initial = target.load(load_ord);
                    let res = target.compare_and_swap(
                        initial.clone(),
                        regs[src].clone(),
                        cas_ord,
                    );
                    if Arc::ptr_eq(&initial, &res) {
                        break;
                    }
                }
            },

            local::CX => {
                let cmp = get_reg!();
                let src = get_reg!();
                let dest = get_reg!();
                let succ = get_ord!(rw);
                let fail = Relaxed;
                regs[dest] = match target.compare_exchange(
                    regs[cmp].clone(),
                    regs[src].clone(),
                    succ,
                    fail,
                ) {
                    Ok(x) => x,
                    Err(x) => x,
                };
            },

            local::LOOP_CX => {
                let src = get_reg!();
                let load_ord = get_ord!(r);
                let succ_ord = get_ord!(rw);
                let fail_ord = Relaxed;
                let mut initial = target.load(load_ord);
                loop {
                    match target.compare_exchange(
                        initial.clone(),
                        regs[src].clone(),
                        succ_ord,
                        fail_ord,
                    ) {
                        Ok(_) => break,
                        Err(p) => initial = p,
                    }
                }
            },

            local::CX_WEAK => {
                let cmp = get_reg!();
                let src = get_reg!();
                let dest = get_reg!();
                let succ = get_ord!(rw);
                let fail = Relaxed;
                regs[dest] = match target.compare_exchange_weak(
                    regs[cmp].clone(),
                    regs[src].clone(),
                    succ,
                    fail,
                ) {
                    Ok(x) => x,
                    Err(x) => x,
                };
            },

            local::LOOP_CX_WEAK => {
                let src = get_reg!();
                let load_ord = get_ord!(r);
                let succ_ord = get_ord!(rw);
                let fail_ord = Relaxed;
                let mut initial = target.load(load_ord);
                loop {
                    match target.compare_exchange_weak(
                        initial.clone(),
                        regs[src].clone(),
                        succ_ord,
                        fail_ord,
                    ) {
                        Ok(_) => break,
                        Err(p) => initial = p,
                    }
                }
            },

            _ => (),
        }
    }

    fn get_read_ord(byte: u8) -> Ordering {
        match byte % 3 {
            0 => SeqCst,
            1 => Acquire,
            2 => Relaxed,
            _ => unreachable!(),
        }
    }

    fn get_rw_ord(byte: u8) -> Ordering {
        match byte % 5 {
            0 => SeqCst,
            1 => Acquire,
            2 => AcqRel,
            3 => Relaxed,
            4 => Release,
            _ => unreachable!(),
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
