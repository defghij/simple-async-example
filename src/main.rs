mod chores;
mod runtime;

use futures::future;




// Look to this for an exampe:
//   https://github.com/aochagavia/async-shenanigans/blob/9368b074c77fd9c3527c50388b054e5bda61597f/src/executor.rs
//
// Introduction to Async:
//   https://jacko.io/async_intro.html
fn main() {
    let mut executor = runtime::init();
    executor.block_on(async {
        println!("Program starting");

        //let mut breakfast = chores::Breakfast::new();
        //breakfast.prepare();
        //assert!(breakfast.is_made());

        //let mut laundry = chores::Laundry::new();
        //laundry.undertake();
        //assert!(laundry.is_done());

        let breakfast = chores::BreakfastFuture::new();
        let laundry = chores::LaundryFuture::new();
        
        //breakfast.await;
        //laundry.await;

        let chores = future::join(breakfast, laundry);
        chores.await;
    });
}
