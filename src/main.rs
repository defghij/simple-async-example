mod chores;
use chores::{
    Choreable,
    simple::{Breakfast, Laundry},
    intermediate::AroundTheHouse
};

use smol::{
    Task,
    LocalExecutor
};
use futures_lite::prelude::*;

use clap::{
    Arg, 
    ArgAction,
    ArgMatches,
    Command,
};


/* # Goal of this Application
 *
 * The goal is to provide a basic set of examples of async usage and implementation that can be
 * pulled from in the future (hehe) as needed. These is not a complete or even a good set of
 * examples but the hope is that is it sufficient to build up a mental model to enable further
 * development with async. Or at least consideration of async.
 *
 * 
 *
 * # Reading Path
 *
 * The suggested way to go through this set of examples is to first look at `main` and
 * `do_the_chores` below. Then open _chores.rs_ and look through the trait `Choreable`,
 * `chores::simple::Breakfast`, then `chores::intermediate::AroundTheHouse` and 
 * `chores::intermediate::trash`.
 * 
 *
 * # Useful resource
 *
 *   Very through introduction to Async.
 *   - https://jacko.io/async_intro.html
 *
 *   Async fib and squares:
 *      - https://github.com/aochagavia/async-shenanigans
 *
 *   Async as State Machines:
 *      - https://www.eventhelix.com/rust/rust-to-assembly-async-await/ 
 *
 *   Async as FSM composition and control inversion:
 *      - https://cliffle.com/blog/async-inversion/
 */
fn main() {
    let args = get_arguments();
    setup_tracing();
    
    // Need to block on `do_the_chores` or main will exit before they finish.
    let _completed = smol::block_on(async {
        do_the_chores(args).await
    });
}

async fn do_the_chores(args: ArgMatches) -> Vec<bool> {
    // Create a single-threaded executor. There is no _parallelism_ here-- just concurrency.
    let local_executor = LocalExecutor::new();
    let mut futures = Vec::new(); //[breakfast, laundry, around_the_house];
    let mut results: Vec<bool> = Vec::new();
    let mut chores = vec![];

    if *args.get_one("first").unwrap() { 
        tracing::info!("setting up simple tasks");
        // This does _not_ run the chore, merely defines it as a task to be awaited on later.
        let breakfast: Task<bool> = smol::spawn(async {
            // Breakfast is a simple chore-- that is it is flat and uses only the `async` keyword.
            <Breakfast as Choreable>::new().start().await
        });
        futures.push(breakfast); // Push Task<bool> to be used later
        results.push(false);     // Set default result for function return

        let laundry: Task<bool> = smol::spawn(async {
            // Laundry is a simple chore-- that is it is flat and uses only the `async` keyword.
            <Laundry as Choreable>::new().start().await 
        });
        futures.push(laundry);
        results.push(false);
    }

    if *args.get_one("second").unwrap() { 
        tracing::info!("setting up intermediate tasks");
        let around_the_house: Task<bool> = smol::spawn(async {
            // This task is in the advanced module because it nests an Async Task within
            // another. The nested Task is taking out the trash and provides
            // its own implementation of the Future trait.
            <AroundTheHouse as Choreable>::new().start().await
        });
        futures.push(around_the_house);
        results.push(false);
    }

    if *args.get_one("third").unwrap() { 
        todo!("no advanced asnync tasks to demonstrate");
    }


    if !futures.is_empty()  {
        tracing::info!("chores starting");

        // Spawn all of the futures (chores) onto the executor at once.
        local_executor.spawn_many(futures, &mut chores);

        // Await all of them and collect the results.
        results = local_executor.run( async move {
            smol::stream::iter(chores).then(|chore| chore).collect::<Vec<_>>().await
        }).await;

        tracing::info!("chores finished");
    }

    results
}

fn get_arguments() -> ArgMatches {
    let args: ArgMatches = Command::new("Async Examples")
        .about("A collection of basic Async examples.")
        .version("0.5.0")
        .author("defghij")
        .arg(
            Arg::new("first")
                .long("simple")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Run the simple async tasks")
        )
        .arg(
            Arg::new("second")
                .long("intermediate")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Run the intermediate async tasks")
        )
        .arg(
            Arg::new("third")
                .long("advanced")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Run the advanced async tasks")
        ).get_matches();
    args
}

fn setup_tracing() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        .with_max_level(tracing::Level::INFO)
        // Build the subscriber
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Subscriber setup should succeed");
}
