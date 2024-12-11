mod runtime;
use std::{cell::RefCell, future::Future, rc::Rc, task::Poll};
use runtime::reactor;

#![feature(future_join)]
use std::future::join;


#[derive(Default)]
pub struct Unit {
    state: u64,
}
type UnitRef = Rc<RefCell<Unit>>;

struct UnitFuture {
    unit: UnitRef,
    terminate_condition: u64,
    id: usize,
    sample_rate: u64
} impl UnitFuture {
    fn new(stop: u64, sample_rate: u64) -> UnitFuture {
        let id = reactor().next_id();
        println!("id given {id}");
        let unit = Unit { state: 0 };
        UnitFuture {
            unit: Rc::new(RefCell::new(unit)),
            terminate_condition: stop,
            id, 
            sample_rate
        }
    }
    fn time_delayed_poll_condition(&self) {
        let id = self.id;
        let rate = self.sample_rate;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(rate));
            runtime::reactor().wake_by_id(id);
        });

    }
} impl Future for UnitFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // One time initialization of the Future
        if self.unit.borrow().state == 0 {
            println!("FIRST POLL - START OPERATION");
            runtime::reactor().set_waker(cx, self.id);
        }

        let unit_state: u64 = self.unit.borrow().state;

        if unit_state != self.terminate_condition {
            self.unit.borrow_mut().state += 1; 
            self.time_delayed_poll_condition();
            println!("pending {}", self.id);

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}


pub struct Breakfast {
    eggs: bool,
    toast: bool,
    sausage: bool
}
type BreakfastRef = Rc<RefCell<Breakfast>>;

struct BreakfastFuture {
    breakfast: BreakfastRef,
    id: usize,
} impl UnitFuture {
    fn new(stop: u64, sample_rate: u64) -> UnitFuture {
        let id = reactor().next_id();
        println!("[Task::{id}] Created Breakfast Task");
        let breakfast = Breakfast {
            eggs: false, toast: false, sausage: false
        }
        BreakfastFuture {
            breakfast: Rc::new(RefCell::new(breakfast)),
            id, 
        }
    }
} impl Future for UnitFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

// Look to this for an exampe:
//   https://github.com/aochagavia/async-shenanigans/blob/9368b074c77fd9c3527c50388b054e5bda61597f/src/executor.rs
fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

async fn async_main() {
    println!("Program starting");
    let task1 = UnitFuture::new(1, 4);
    let task2 = UnitFuture::new(4, 1);

    join!(task1, task2).await;


}

