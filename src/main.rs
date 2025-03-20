mod chores;

use smol::{
    Task,
    LocalExecutor
};
use futures_lite::prelude::*;


/* Useful links:
 *   https://jacko.io/async_intro.html
 *   https://github.com/aochagavia/async-shenanigans/blob/9368b074c77fd9c3527c50388b054e5bda61597f/src/executor.rs
 */

fn main() {
    println!("Going to start the chores");

    // Need to block on `do_the_chores` or main will exit before they finish.
    let completed = smol::block_on(async {
        do_the_chores().await
    });

    println!("All chores completed {:?}", completed);

}

async fn do_the_chores() -> Vec<bool> {
    // Create a single-threaded executor. There is no _parallelism_ here-- just concurrency.
    let local_executor = LocalExecutor::new();

    // This does _not_ run the chore, merely defines it as a task to be awaited on later.
    let breakfast: Task<bool> = smol::spawn(async {
       let mut breakfast = chores::Breakfast::new(); 
       breakfast.prepare().await;
       breakfast.is_made()
    });

    let laundry: Task<bool> = smol::spawn(async {
       let mut laundry = chores::Laundry::new(); 
       laundry.undertake().await;
       laundry.is_done()
    });

    let around_the_house: Task<bool> = smol::spawn(async {
       let mut around_the_house = chores::AroundTheHouse::new();
       around_the_house.conduct().await;
       around_the_house.is_finished()
    });

    // Spawn all of the futures (chores) onto the executor at once.
    let futures = [breakfast, laundry, around_the_house];
    let mut chores = vec![];
    local_executor.spawn_many(futures, &mut chores);

    // Await all of them.
    let results: Vec<bool> = local_executor.run( async move {
        smol::stream::iter(chores).then(|chore| chore).collect::<Vec<_>>().await
    }).await;

    results
}
