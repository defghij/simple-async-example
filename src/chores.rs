use std::{pin::Pin, time::Duration};
use async_io::Timer;
use tracing;

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

/// This trait is to abstract the external functions of a Chore. That is, creating one, starting
/// it, and then running it. The goal of pulling these out into a Trait is to first show there is a
/// common abstraction to these and second to make this implementation distinct from the
/// implementation of executing the sub-tasks of a chore. Thus, for `chores::{simple,intermediate}`,
/// the `impl <Type> {...}` are 1) all async 2) only functions related to running the chores
/// sub-task. All other functionality for the purpose of this set of examples resides here in the
/// Choreable trait.  
pub trait Choreable {
    type Output;
    /// Create a new Chore of type Output. 
    fn new() -> Self::Output; 

    /// Start the Chore's sub-tasks and return the result. In this case it is a `bool` that
    /// indicates that all sub-tasks have completed.
    async fn start(&mut self) -> bool;

    /// Very simple function to simulate being blocked on an external resource or task.
    /// It task two arguments: a task_name and a duration for the task.
    /// This function spawns a thread with a sleep. When the sleep finishes the task is 
    /// considered complete.
    ///
    /// The return value (always true) is to mimic some return value of a real non-blocking
    /// function that an async task might invoke.
    async fn run(task_name: &str, duration: Duration) -> bool {
        tracing::info!("{}, started", task_name);

        //let task = std::thread::spawn(move || {
                //std::thread::sleep(std::time::Duration::from_secs(duration.as_secs()));
            //});
        Timer::after(duration).await;
        tracing::info!("{}, finished ({}s)", task_name, duration.as_secs());
        true
    }
}


/// # Intro 
/// This module contains two chores:
/// - Breakfast
/// - Laundry
///
/// Both are flat in that they do not nest any async tasks. Additionally, they proceed in a linear
/// manner and make use of the `async` keyword. They should be mostly straight forward and have the
/// same structure.
///
/// # Struct
/// The structure holds the state (boolean fields) that indicate whether the task has
/// accomplished its goal-- completing the associated "chore". Each sub-chore has a field in the
/// structure that starts as false and is set to true when the sub-chore has been finished.
///
/// # impl <Type>
///
/// This contains individual async functions tied to each sub-task. In the case of
/// simple::Breakfast we have the following:
/// ```
///     async fn scamble_eggs(&mut self) { ... }
///     async fn toast_bread(&mut self) { ... }
///     async fn fry_sausage(&mut self) { ... }
///     async fn pour_orange_juice(&mut self) { ... }
/// ```
/// Each of these will be executed in the order presented her by 
/// `<Breakfast as Choreable>::start()`-- that is, `.start()` call each of these async
/// functions in this order. Each of these functions has a very simple form: 
/// ```
///     async fn scamble_eggs(&mut self) {
///
///         // use a non-blocking timer, set to 3s, to "run" the task. 
///         <Breakfast as Choreable>::run("[task 1.a] breakfast.scramble_eggs", Duration::new(3,0)).await;
///
///         // Set the appropriate field to indicate the sub-task has completed.
///         self.eggs = true;
///     }
///```
/// Each task is given a timer duration that ensures that sub-tasks, across Tasks 
/// (chores::simple::{Breakfast, Laundry}) complete in a staggered or interleaved fashion. 
///
/// Note that we could have sub-tasks call their own sub-tasks. That kind of behavior is confined
/// to the `chores::intermediate` module though`.
///
/// # impl Choreable for <Type>
/// 
/// See `Choreable` trait above.
pub mod simple {
    use super::*;

    /// Collection of chores that represent preparing breakfast. 
    #[derive(Debug)]
    pub struct Breakfast {
        eggs: bool,
        toast: bool,
        sausage: bool,
        orange_juice: bool,
    } 
    impl Breakfast {

        #[tracing::instrument(level = "debug")]
        async fn scamble_eggs(&mut self) {
            self.eggs =
                <Breakfast as Choreable>::run("[task 1.a] breakfast.scramble_eggs", Duration::new(3,0)).await;
        }

        #[tracing::instrument(level = "debug")]
        async fn toast_bread(&mut self) {
            self.toast = 
                <Breakfast as Choreable>::run("[task 1.b] breakfast.toast_bread", Duration::new(1,0)).await;
        }

        #[tracing::instrument(level = "debug")]
        async fn fry_sausage(&mut self) {
            self.sausage = 
                <Breakfast as Choreable>::run("[task 1.c] breakfast.fry_sausage", Duration::new(6,0)).await;
        }

        #[tracing::instrument(level = "debug")]
        async fn pour_orange_juice(&mut self) {
            self.orange_juice =
                <Breakfast as Choreable>::run("[task 1.d] breakfast.pour_orange_juice", Duration::new(1,0)).await;
        }
    }
    impl Choreable for Breakfast {
        type Output = Breakfast;

        fn new() -> Self::Output {
            Breakfast {
                eggs: false,
                toast: false,
                sausage: false,
                orange_juice: false,
            }
        }

        #[tracing::instrument(level = "debug")]
        async fn start(&mut self) -> bool {
            self.scamble_eggs().await;
            self.toast_bread().await;
            self.fry_sausage().await;
            self.pour_orange_juice().await;
            self.eggs && self.orange_juice && self.sausage && self.toast
        }
    }

    #[derive(Debug)]
    pub struct Laundry {
        picked_up: bool,
        washed: bool,
        dried: bool,
        folded: bool,
        put_away: bool,
    }
    impl Laundry {
        #[tracing::instrument(level = "debug")]
        async fn pickup(&mut self) {
            self.picked_up = 
                <Laundry as Choreable>::run("[task 2.a] laundry.pick up", Duration::new(1,0)).await;
        }

        #[tracing::instrument(level = "debug")]
        async fn wash(&mut self) {
            <Laundry as Choreable>::run("[task 2.b] laundry.wash", Duration::new(6,0)).await;
            self.washed = true;
        }

        #[tracing::instrument(level = "debug")]
        async fn dry(&mut self) {
            <Laundry as Choreable>::run("[task 2.c] laundry.dry", Duration::new(4,0)).await;
            self.dried = true;
        }

        #[tracing::instrument(level = "debug")]
        async fn fold(&mut self) {
            <Laundry as Choreable>::run("[task 2.d] laundry.fold", Duration::new(4,0)).await;
            self.folded = true;
        }

        #[tracing::instrument(level = "debug")]
        async fn put_away(&mut self) {
            <Laundry as Choreable>::run("[task 2.e] laundry.put_away", Duration::new(2,0)).await;
            self.put_away = true;
        }
    }
    impl Choreable for Laundry {
        type Output = Laundry;
        fn new() -> Laundry {
            Laundry {
                picked_up: false,
                washed: false,
                dried: false,
                folded: false,
                put_away: false,
            }
        }

        #[tracing::instrument(level = "debug")]
        async fn start(&mut self) -> bool {
            self.pickup().await;
            self.wash().await;
            self.dry().await;
            self.fold().await;
            self.put_away().await;
            self.picked_up && self.washed && self.dried && self.folded && self.put_away       
        }
    }
}

/// # Introduction
///
/// This module increases the complexity slightly by doing two things. First is by nesting async
/// Tasks. Within the `AroundTheHouse` task is another task called `trash`. We can see this by the
/// way it is called in the `impl AroundTheHouse`:
/// ```
///      async fn take_out_trash(&mut self) {
///          let trash = trash::Future::new();
///          self.trash_taken_out = trash.await
///      }
/// ```
/// What is happening here is that we're creating a new future and then `.await`ing it. The `trash`
/// chore future increasing the complexity by implementing the `Future` trait. 
pub mod intermediate {
    use super::*;

    /// Collection of chores from around the house-- take out the trash and water plants. Taking out
    /// the trash is a multi-step sub-task. See the `take_out_trash` function.
    #[derive(Debug)]
    pub struct AroundTheHouse {
        trash_taken_out: bool,
        plants_watered: bool
    }
    impl AroundTheHouse {
        #[tracing::instrument(level = "debug")]
        async fn take_out_trash(&mut self) {
            let trash = trash::Future::new();
            self.trash_taken_out = trash.await
        }

        #[tracing::instrument(level = "debug")]
        async fn water_plants(&mut self) { 
            self.plants_watered = 
                <AroundTheHouse as Choreable>::run("[task 3.b] around-the-house.plants", Duration::new(3,0)).await;
        }
    }
    impl Choreable for AroundTheHouse {
        type Output = AroundTheHouse;

        fn new() -> Self::Output {
            AroundTheHouse {
                trash_taken_out: false,
                plants_watered: false
            }
        }

        #[tracing::instrument(level = "debug")]
        async fn start(&mut self) -> bool {
            self.take_out_trash().await;
            self.water_plants().await;
            self.trash_taken_out && self.plants_watered
        }
    }

    /// # Enum: State
    /// In this module we implement a "trash" chore by implementing the `Future` trait. 
    /// The `Future` trait makes use of an enum called `State` to track where in the chore
    /// sub-tasks the task is in. 
    ///
    /// # Struct: Future
    ///
    /// The "Trash" chore structure maintains the state that is needed across calls to `poll` used
    /// by the (Smol) executor. The state needed are the current position in the state machine the
    /// task is in and function that will be used to determine when the next state transition
    /// should occur. This is analogous to the `Choreable::run` function from `chores::simple`.
    ///
    /// # Trait: Future
    /// 
    /// The `Future` trait implementation is a state machine that has the following form:
    /// ```
    ///  [ Start ] --> [ Gather ] --> [ TakeOut ] -> [ Done ]
    /// (enter here)                              (finish here)
    /// ```
    /// Within the implementation the state is transitions are mostly the same between each state.
    /// For this reason the implementation extracts out the login into a closure with the hope that
    /// the state machine reads a bit clearer. 
    pub mod trash {
        use super::*;

        /// Enumerate the different states that this task can be in.
        enum State {
            /// Initial state of the state machine (Poll::Pending)
            Start,
            Gather,
            TakeOut,
            /// Final state of the machine (Poll::Ready)
            Done
        }
        impl State {
            pub fn to_str(&self) -> &'static str {
                match self {
                    State::Start   => "trash.start",
                    State::Gather  => "trash.gather",
                    State::TakeOut => "trash.take_out",
                    State::Done    => "trash.done",
                }
            }
            pub fn to_stage(&self) -> &'static str {
                match self {
                    State::Start   => "i",
                    State::Gather  => "ii",
                    State::TakeOut => "iii",
                    State::Done    => "iv",
                }
            }
        }

        /// This is the structure that maintains state across invocations of `poll`. 
        pub struct Future {
            /// Current state of the FSM
            state: State,
            /// Timer which can be used as a proxy for when the sub-task is ready. Must be Box so
            /// that is lives on the heap. Is Pin so that this memory does not move.
            sleep: Pin<Box<Timer>>
        }
        impl Future {
            pub fn new() -> Future {
                Future {
                    // We create our future in the Start state.
                    state: State::Start,

                    // Set duration to zero so the first time the Future is polled it enters the start
                    // state and immediately transitions to the next non-start state
                    sleep: Box::pin(Timer::after(Duration::from_secs(0))),
                }
            }
        }
        impl futures::Future for Future {
            type Output = bool;

            /// This is the function that will be called by the executor to _drive_ the future
            /// forward. First we have a closure that extracts out the simple and common
            /// functionality between the different states. 
            ///
            /// Then we have a match statement that will select the appropriate set of actions
            /// given the current `state` of the FSM. 
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {

                // Define a closure to extract out the state-change logic.
                let try_state_change = |mut s: Pin<&mut Self>,
                                        cx:&mut std::task::Context<'_>, 
                                        next_state: State,
                                        state_length_in_seconds: Duration|
                {
                    // We check if our sleep timer is ready meaning we can change states.
                    if s.sleep.as_mut().poll(cx).is_ready() {
                        println!("[Task 3.a.{}] around-the-house.{} --> {}",s.state.to_stage(),
                                                           s.state.to_str(),
                                                           next_state.to_str());

                        s.as_mut().state = next_state; // Change to the next state of the Task.

                        // Set waker for next state. This amounts to how much time we want to be in the
                        // next_state for. 
                        s.as_mut().sleep.set(Timer::after(state_length_in_seconds));
                        
                        cx.waker().wake_by_ref(); // Inform the reactor we want woken up.
                        
                        std::task::Poll::<bool>::Pending // Return Pending for the over all Task.
                    } 
                    else {
                        // The timer for state-transition is not ready yet so we return pending.
                        std::task::Poll::<bool>::Pending
                    }
                };

                // Heart of the FSM that forms the foundation of this async task.
                match self.state {
                    State::Start => {
                        // Start --> Gather is instaneous.
                        // Set duration of Gather to 4 seconds.
                        try_state_change(self, cx, State::Gather, Duration::from_secs(4))
                    },
                    State::Gather => {
                        // Gather --> TakeOut
                        // Set duration of TakeOut to 4 seconds.
                        try_state_change(self, cx, State::TakeOut, Duration::from_secs(4))
                    },
                    State::TakeOut => {
                        // TakeOut --> Done
                        // Set duration of Done to 0 seconds.
                        try_state_change(self, cx, State::Done, Duration::from_secs(0))
                    },
                    State::Done => {
                        println!("[Task 3.a] around-the-house.trash.done");
                        std::task::Poll::Ready(true)
                    }
                }
            }
        }
    }
}

pub mod advanced {

}
