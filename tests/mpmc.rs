extern crate lockfree;

use lockfree::prelude::*;

#[test]
fn single_threaded_order() {
    let (sender, receiver) = channel();
    assert_eq!(receiver.recv(), None);
    sender.send(5);
    assert_eq!(receiver.recv(), Some(5));
    sender.send(12);
    sender.send(124);
    assert_eq!(receiver.recv_wait(), 12);
    assert_eq!(receiver.recv(), Some(124));
    assert_eq!(receiver.recv(), None);
}
