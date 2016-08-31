extern crate tokio_core;
extern crate futures;

use futures::Future;
use tokio_core::Loop;

#[test]
fn simple() {
    let mut lp = Loop::new().unwrap();

    let (tx1, rx1) = futures::oneshot();
    let (tx2, rx2) = futures::oneshot();
    lp.pin().spawn(futures::lazy(|| {
        tx1.complete(1);
        Ok(())
    }));
    lp.handle().spawn(|_| {
        futures::lazy(|| {
            tx2.complete(2);
            Ok(())
        })
    });

    assert_eq!(lp.run(rx1.join(rx2)).unwrap(), (1, 2));
}
