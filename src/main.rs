mod chores;

use smol::{
    Task,
    LocalExecutor
};
use futures_lite::prelude::*;


//let my_stream = smol::stream::iter(vec![1u32, 2, 3]);
//
//// Spawn the set of futures on an executor.
//let handles: Vec<smol::Task<()>> = my_stream
//    .map(|item| {
//        // Spawn the future on the executor.
//        ex.spawn(do_something(item))
//    }).collect().await;
//
//// Wait for all of the handles to complete.
//for handle in handles {
//    handle.await;
//}



// Look to this for an exampe:
//   https://github.com/aochagavia/async-shenanigans/blob/9368b074c77fd9c3527c50388b054e5bda61597f/src/executor.rs
//
// Introduction to Async:
//   https://jacko.io/async_intro.html
fn main() {
    let _completed = smol::block_on(async {
        do_the_chores().await
    });

}

async fn do_the_chores() -> Vec<bool> {
    let local_executor = LocalExecutor::new();

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

    // Spawn all of the futures onto the executor at once.
    let futures = [breakfast, laundry];
    let mut tasks = vec![];
    local_executor.spawn_many(futures, &mut tasks);

    // Await all of them.
    let results = local_executor.run(async move {
        smol::stream::iter(tasks).then(|x| x).collect::<Vec<_>>().await
    }).await;
    results
}
