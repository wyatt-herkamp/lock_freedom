extern crate lockfree;

use lockfree::darc::{Darc, Ordering::*};
use std::{sync::Arc, thread};

#[test]
fn create_and_drop() {
    Darc::from("abc".to_owned());
}

#[test]
fn use_and_set() {
    let darc = Darc::from("abc".to_owned());
    for _ in 0..20 {
        darc.load(SeqCst);
        let old = darc.swap(Arc::new("123".to_owned()), SeqCst);
        let new = Arc::new("hahaha".to_owned());
        darc.compare_and_swap(old.clone(), new.clone(), SeqCst);
        let real = darc.compare_and_swap(old.clone(), new.clone(), SeqCst);
        darc.compare_and_swap(real, Arc::new("hahaha".to_owned()), SeqCst);
    }
}

#[test]
fn multithread_use_and_set() {
    let darc = Arc::new(Darc::from("abc".to_owned()));
    let mut threads = Vec::with_capacity(20);
    for i in 0..20 {
        let darc = darc.clone();
        threads.push(thread::spawn(move || {
            let init = darc.load(SeqCst);
            let prefix = darc.swap(Arc::from(format!("{}{}", init, i)), SeqCst);
            for j in 0..10 {
                loop {
                    let inner = darc.load(SeqCst);
                    let new =
                        Arc::new(format!("{}{}{}{}", prefix, inner, i, j));
                    let res = darc.compare_and_swap(inner.clone(), new, SeqCst);
                    if Arc::ptr_eq(&res, &inner) {
                        break;
                    }
                }
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
