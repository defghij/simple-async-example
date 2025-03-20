//use std::{
//    cell::RefCell,
//    future::Future,
//    rc::Rc,
//    task::Poll, thread
//};
//use std::sync::atomic::{AtomicBool, Ordering};
//use std::sync::Arc;
use std::time::Duration;
use async_io::Timer;

//use super::runtime::reactor;

/* This file contains four chores ("tasks"): Breakfast, Laundry, AroundTheHouse, and Trash.
 * Breakfast and Laundry and independent of the other chores.
 * Each of the tasks demonstrates a different thing:
 *     - Breakfast simply using the `async` keyword
 *     - AroundTheHouse depends on Trasn to demonstrate a simple nesting of async tasks 
 *     - Laundry implements its own Future.
 *
 *
 * Each chore is implemented as their own structure though they share the same form. The fields of
 * the structure are boolean values indicating whether the sub-task has finished. There are three
 * associated functions the construct, start, and then return the status of the chore. 
 *
 * There are also async function for completing each sub-task. The sub-tasks make use of a non-blocking
 * timer. This is to simulate time in which the task yields to the operating system or device for
 * some task.
 *
 */


/// Very simple function to simulate being blocked on an external resource or task.
/// It task two arguments: a task_name and a duration for the task.
/// This function spawns a thread with a sleep. When the sleep finishes the task is 
/// considered complete.
pub async fn run(task_name: &str, duration: Duration) {
    println!("{task_name}, started");

    //let task = std::thread::spawn(move || {
            //std::thread::sleep(std::time::Duration::from_secs(duration.as_secs()));
        //});
    Timer::after(duration).await;

    println!("{task_name}, finished ({}s)", duration.as_secs());
}

/// Collection of chores that represent preparing breakfast. 
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

    /// The order in which we prepare breakfast.
    pub async fn prepare(&mut self) {
        self.scamble_eggs().await;
        self.toast_bread().await;
        self.fry_sausage().await;
        self.pour_orange_juice().await;
    }

    /// This is the result of the chore-- we return a boolean indicating we've finished.
    pub fn is_made(&self) -> bool {
        self.eggs && self.orange_juice && self.sausage && self.toast
    }

    async fn scamble_eggs(&mut self) {
        run("[task 1.a] breakfast.scramble_eggs", Duration::new(3,0)).await;
        self.eggs = true;
    }

    async fn toast_bread(&mut self) {
        run("[task 1.b] breakfast.toast_bread", Duration::new(1,0)).await;
        self.toast = true;
    }

    async fn fry_sausage(&mut self) {
        run("[task 1.c] breakfast.fry_sausage", Duration::new(6,0)).await;
        self.sausage = true;
    }

    async fn pour_orange_juice(&mut self) {
        run("[task 1.d] breakfast.pour_orange_juice", Duration::new(1,0)).await;
        self.orange_juice = true;
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

    pub async fn undertake(&mut self) {
        self.pickup().await;
        self.wash().await;
        self.dry().await;
        self.fold().await;
        self.put_away().await;
    }

    pub fn is_done(&self) -> bool {
           self.picked_up && self.washed && self.dried && self.folded && self.put_away       
    }

    async fn pickup(&mut self) {
        run("[task 2.a] laundry.pick up", Duration::new(1,0)).await;
        self.picked_up = true;
    }

    async fn wash(&mut self) {
        run("[task 2.b] laundry.wash", Duration::new(6,0)).await;
        self.washed = true;
    }

    async fn dry(&mut self) {
        run("[task 2.c] laundry.dry", Duration::new(4,0)).await;
        self.dried = true;
    }

    async fn fold(&mut self) {
        run("[task 2.d] laundry.fold", Duration::new(4,0)).await;
        self.folded = true;
    }

    async fn put_away(&mut self) {
        run("[task 2.e] laundry.put_away", Duration::new(2,0)).await;
        self.put_away = true;
    }
}

/// Collection of chores from around the house-- take out the trash and water plants. Taking out
/// the trash is a multi-step sub-task. See the `take_out_trash` function.
pub struct AroundTheHouse {
    trash_taken_out: bool,
    plants_watered: bool
}
impl AroundTheHouse {
    pub fn new() -> AroundTheHouse {
        AroundTheHouse {
            trash_taken_out: false,
            plants_watered: false
        }
    }

    pub async fn conduct(&mut self) { 
        self.take_out_trash().await;
        self.water_plants().await;
    }

    pub fn is_finished(&self) -> bool {
           self.trash_taken_out && self.plants_watered
    }

    /// Utilize the Trash chore. Note that the duration is 0 because the Trash chore has its own
    /// sub-task duration.
    async fn take_out_trash(&mut self) {
        let mut trash = Trash::new(); 
        run("[task 3.a] around-the-house.trash", Duration::new(0,0)).await;
        self.trash_taken_out = trash.conduct().await
    }

    async fn water_plants(&mut self) { 
        run("[task 3.b] around-the-house.plants", Duration::new(3,0)).await;
        self.plants_watered = true;
    }
}

/// A structure to denote the task of doing the Trash chore. Sub-tasks are gathering up the trash
/// and then taking it out. 
pub struct Trash {
    gathered: bool,
    taken_out: bool
}
impl Trash {
    pub fn new() -> Trash {
        Trash {
            gathered: false,
            taken_out: false,
        }
    }

    pub async fn conduct(&mut self) -> bool { 
        self.gather().await;
        self.take_out().await;
        self.is_complete()
    }

    fn is_complete(&self) -> bool {
           self.gathered && self.taken_out
    }

    async fn gather(&mut self) {
        run("[task 4.a] trash.gather", Duration::new(3,0)).await;
        self.gathered = true;
    }

    async fn take_out(&mut self) {
        run("[task 4.b] trash.take_out", Duration::new(3,0)).await;
        self.taken_out = true;
    }
}
