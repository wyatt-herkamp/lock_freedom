extern crate lockfree;

use lockfree::prelude::*;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    thread,
};

#[test]
fn single_threaded_order() {
    let stack = Stack::new();
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
    stack.push(3);
    stack.push(5);
    assert_eq!(stack.pop(), Some(5));
    stack.push(7);
    stack.push(8);
    assert_eq!(stack.pop(), Some(8));
    assert_eq!(stack.pop(), Some(7));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
    stack.push(2);
    stack.push(0);
    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn no_leak() {
    let stack = Stack::new();
    stack.push("foo".to_string());
    stack.push("bar".to_string());
    assert_eq!(stack.pop(), Some("bar".to_string()));
}

#[test]
fn multithreaded() {
    generic_multithreaded(20, 800, 55);
    generic_multithreaded(50, 10000, 87);
}

fn generic_multithreaded(nthread: usize, niter: usize, nmod: usize) {
    let stack = Arc::new(Stack::new());
    let mut handles = Vec::with_capacity(nthread);
    let _dbg = Arc::new(AtomicUsize::new(0));
    for i in 0..nthread {
        let stack = stack.clone();
        let _dbg = _dbg.clone();
        handles.push(thread::spawn(move || {
            for j in 0..niter {
                let val = (i * niter) + j;
                stack.push(val);
                if (val + 1) % nmod == 0 {
                    stack.pop();
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let expected = niter * nthread - niter * nthread / nmod;
    let mut res = 0;
    while let Some(val) = stack.pop() {
        assert!(val < niter * nthread);
        res += 1;
    }
    assert_eq!(res, expected);
}
