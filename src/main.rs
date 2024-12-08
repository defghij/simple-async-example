mod http;
mod runtime;
use std::{cell::RefCell, future::Future, rc::Rc, task::Poll};
use runtime::reactor;


#[derive(Default)]
pub struct Unit {
    state: u64,
}
type UnitRef = Rc<RefCell<Unit>>;

struct UnitFuture {
    unit: UnitRef,
    terminate_condition: u64,
    id: usize
} impl UnitFuture {
    fn new(stop: u64) -> UnitFuture {
        let id = reactor().next_id();
        let unit = Unit { state: 0 };
        UnitFuture {
            unit: Rc::new(RefCell::new(unit)),
            terminate_condition: stop,
            id
        }
    }
} impl Future for UnitFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // Initial Conditions
        if self.unit.borrow().state == 0 {
            println!("FIRST POLL - START OPERATION");
            runtime::reactor().set_waker(cx, self.id);
        }

        let unit_state: u64 = self.unit.borrow().state;

        if unit_state != self.terminate_condition {
            self.unit.borrow_mut().state += 1; 
            let id = self.id;

            // Spawn a thread to wake our task up in the future.
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(2));
                runtime::reactor().wake_by_id(id);
            });

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}




fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

async fn async_main() {
    println!("Program starting");
    
    UnitFuture::new(10).await;
}

