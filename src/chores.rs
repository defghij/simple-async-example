use std::{
    cell::RefCell,
    future::Future,
    rc::Rc,
    task::Poll, thread
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use super::runtime::reactor;

/// Very simple function to simulate being blocked on an external resource or task.
/// It task two arguments: a task_name and a duration for the task.
/// This function spawns a thread with a sleep. When the sleep finishes the task is 
/// considered complete.
pub fn run(task_name: &str, duration: Duration) {
    println!("Starting {task_name} (duration: {} s)", duration.as_secs());

    let task = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(duration.as_secs()));
        });

    let _ = task.join();

    println!("Finished {task_name}");
}

pub struct Breakfast {
    eggs: bool,
    toast: bool,
    sausage: bool,
    orange_juice: bool,
} 
impl Breakfast {
    pub fn new() -> Breakfast {
        Breakfast {
            eggs: false,
            toast: false,
            sausage: false,
            orange_juice: false,
        }
    }

    pub fn prepare(&mut self) {
        self.scamble_eggs();
        self.toast_bread();
        self.fry_sausage();
        self.pour_orange_juice();
    }

    pub fn is_made(&self) -> bool {
        self.eggs && self.orange_juice && self.sausage && self.toast
    }

    fn scamble_eggs(&mut self) {
        run("breakfast.scramble_eggs", Duration::new(2,0));
        self.eggs = true;
    }

    fn toast_bread(&mut self) {
        run("breakfast.toast_bread", Duration::new(1,0));
        self.toast = true;
    }

    fn fry_sausage(&mut self) {
        run("breakfast.fry_sausage", Duration::new(3,0));
        self.sausage = true;
    }

    fn pour_orange_juice(&mut self) {
        run("breakfast.pour_orange_juice", Duration::new(0,500_000));
        self.orange_juice = true;
    }

}
type BreakfastRef = Rc<RefCell<Breakfast>>;

enum BreakfastState {
    Start,
    ScambleEgg,
    ToastBread,
    FrySausage,
    PourJuice,
    Done
}

pub struct BreakfastFuture {
    task: BreakfastRef,
    state: BreakfastState,
    waiting: Arc<AtomicBool>,
    id: usize,
} 
impl BreakfastFuture {
    pub fn new() -> BreakfastFuture { 
        let id = reactor().next_id();
        println!("Breakfast TaskID:{id}");
        BreakfastFuture {
            task: Rc::new(RefCell::new(Breakfast::new())),
            state: BreakfastState::Start,
            waiting: Arc::new(AtomicBool::new(true)),
            id, 
        }
    }

    fn schedule_wake(&mut self, cx: &mut std::task::Context<'_>, duration: Duration) {
        reactor().set_waker(cx, self.id);
        let id = self.id;
        let mut waiting: Arc<AtomicBool> = self.waiting.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            waiting.store(false, Ordering::Relaxed);
            reactor().wake_by_id(id);
        });
        self.waiting.store(true, Ordering::Relaxed);
    }
}
impl Future for BreakfastFuture {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {

        match self.state {
            BreakfastState::Start => { 
                // Breakfast is being started.
                println!("Starting Breakfast Tasks");
                reactor().set_waker(cx, self.id);

                // We begin by starting the Scrambled Eggs which take 2 seconds.
                // This is essentially the state transition from Start -> ScrambleEgg
                // which takes two seconds as timed by another thread.
                self.state = BreakfastState::ScambleEgg;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            BreakfastState::ScambleEgg => {
                // We check to see if our Eggs are done. The amounts to
                // waiting until our timer thread toggles self.waiting
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                // Once the Eggs are done, we set the appropriate state. Then
                // begin the transition to the next state which is the toasting
                // of bread. This state transition takes 1 second.
                self.task.borrow_mut().eggs = true;
                println!("Eggs ready");
                self.state = BreakfastState::ToastBread;
                self.schedule_wake(cx, Duration::from_secs(1));
                Poll::Pending
            },
            BreakfastState::ToastBread => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().toast = true;
                println!("Toast ready");
                self.state = BreakfastState::FrySausage;
                self.schedule_wake(cx, Duration::from_secs(3));
                Poll::Pending
            },
            BreakfastState::FrySausage => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().sausage = true;
                println!("Sauage ready");
                self.state = BreakfastState::PourJuice;
                self.schedule_wake(cx, Duration::from_secs(1));
                Poll::Pending
            },
            BreakfastState::PourJuice => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().orange_juice = true;
                println!("Juice ready");
                self.state = BreakfastState::Done;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            BreakfastState::Done => {
                println!("Breakfast is ready!");
                return Poll::Ready(());
            },
        }
    }
}


pub struct Laundry {
    picked_up: bool,
    washed: bool,
    dried: bool,
    folded: bool,
    put_away: bool,
}
impl Laundry {
    pub fn new() -> Laundry {
        Laundry {
            picked_up: false,
            washed: false,
            dried: false,
            folded: false,
            put_away: false,
        }
    }

    pub fn undertake(&mut self) {
        self.pickup();
        self.wash();
        self.dry();
        self.fold();
        self.put_away();
    }

    pub fn is_done(&self) -> bool {
           self.picked_up && self.washed && self.dried && self.folded && self.put_away       
    }

    fn pickup(&mut self) {
        run("laundry.pick up", Duration::new(0,500_000));
        self.picked_up = true;
    }
    fn wash(&mut self) {
        run("laundry.wash", Duration::new(2,0));
        self.washed = true;
    }
    fn dry(&mut self) {
        run("laundry.dry", Duration::new(3,0));
        self.dried = true;
    }
    fn fold(&mut self) {
        run("laundry.fold", Duration::new(1,0));
        self.folded = true;
    }
    fn put_away(&mut self) {
        run("laundry.put_away", Duration::new(1,0));
        self.put_away = true;
    }
}
type LaundryRef = Rc<RefCell<Laundry>>;

enum LaundryState {
    Start,
    PickedUp,
    Washed,
    Dried,
    Folded,
    PutAway,
    Done
}
pub struct LaundryFuture {
    task: LaundryRef,
    state: LaundryState,
    waiting: Arc<AtomicBool>,
    id: usize,
} 
impl LaundryFuture {
    pub fn new() -> LaundryFuture { 
        let id = reactor().next_id();
        println!("Laundry TaskID:{id}");
        LaundryFuture {
            task: Rc::new(RefCell::new(Laundry::new())),
            state: LaundryState::Start,
            waiting: Arc::new(AtomicBool::new(false)),
            id, 
        }
    }

    fn schedule_wake(&mut self, cx: &mut std::task::Context<'_>, duration: Duration) {
        reactor().set_waker(cx, self.id);
        let id = self.id;
        let mut waiting: Arc<AtomicBool> = self.waiting.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            waiting.store(false, Ordering::Relaxed);
            reactor().wake_by_id(id);
        });
        self.waiting.store(true, Ordering::Relaxed);
    }
}
impl Future for LaundryFuture {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.task.borrow().is_done() == false {
            reactor().set_waker(cx, self.id);
        }
        match self.state {
            LaundryState::Start => { 
                println!("Starting Laundry Tasks");
                reactor().set_waker(cx, self.id);

                self.state = LaundryState::PickedUp;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            LaundryState::PickedUp => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }
                
                self.task.borrow_mut().picked_up = true;
                println!("Laundry picked up");
                self.state = LaundryState::Washed;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            LaundryState::Washed => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().washed = true;
                println!("Laundry washed");
                self.state = LaundryState::Dried;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            LaundryState::Dried => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().dried = true;
                println!("Laundry dried");
                self.state = LaundryState::Folded;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            LaundryState::Folded => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().folded = true;
                println!("Laundry folded");
                self.state = LaundryState::PutAway;
                self.schedule_wake(cx, Duration::from_secs(2));
                Poll::Pending
            },
            LaundryState::PutAway => {
                if self.waiting.load(Ordering::Relaxed) { return Poll::Pending; }

                self.task.borrow_mut().put_away = true;
                println!("Laundry put away");
                self.state = LaundryState::Done;
                self.schedule_wake(cx, Duration::from_secs(1));
                Poll::Pending
            },
            LaundryState::Done => {
                println!("Laundry is Done!");
                return Poll::Ready(());
            },
        }
    }
}
