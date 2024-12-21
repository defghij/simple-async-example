mod chores;

use smol::{
    LocalExecutor,
};
use futures_lite::{
    stream,
    prelude::*
};



// Look to this for an exampe:
//   https://github.com/aochagavia/async-shenanigans/blob/9368b074c77fd9c3527c50388b054e5bda61597f/src/executor.rs
//
// Introduction to Async:
//   https://jacko.io/async_intro.html
fn main() {
    let local_executor = LocalExecutor::new();
    
    let breakfast = chores::BreakfastFuture::new();
    let laundry = chores::LaundryFuture::new();

    let futures = [breakfast, laundry];

    let mut tasks = vec![];
    local_executor.spawn_many(futures,&mut tasks);

    let result = local_executor.run(async move {
        stream::iter(tasks).then(|x| x).collect::<Vec<_>>().await
    }).await;
}
