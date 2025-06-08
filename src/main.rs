use std::env;

use smol::{
    Task,
    LocalExecutor
};
use futures_lite::prelude::*;

use dotenv::dotenv;
use clap::{
    Arg, 
    ArgAction,
    ArgMatches,
    Command,
};

mod chores;
use chores::{
    Choreable,
    simple::{Breakfast, Laundry},
    intermediate::AroundTheHouse
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
    // Do _all_ of the initial setup for the application.
    let args: ArgMatches = initialize_application();

    // Get the kinds of tasks we want to do from
    // the commandline arguments.
    let task_kinds: (bool, bool, bool) = (
        *args.get_one::<bool>("first").expect("Argparse should guarantee a bool"),
        *args.get_one::<bool>("second").expect("Argparse should guarantee a bool"),
        *args.get_one::<bool>("third").expect("Argparse should guarantee a bool"),
    );

    // Need to block on `do_the_chores` or main will exit before they finish.
    let _completed = smol::block_on(async {
        do_the_chores(task_kinds).await
    });
}

async fn do_the_chores(task_kinds: (bool, bool, bool)) -> Vec<bool> {
    // Create a single-threaded executor. There is no _parallelism_ here-- just concurrency.
    let local_executor = LocalExecutor::new();
    let mut futures = Vec::new(); //[breakfast, laundry, around_the_house];
    let mut results: Vec<bool> = Vec::new();
    let mut chores = vec![];

    if task_kinds.0 { 
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

    if task_kinds.1 { 
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

    if task_kinds.2 { 
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

fn setup_tracing(level: tracing::Level) {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(level)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Subscriber setup should succeed");
}

fn initialize_application() -> ArgMatches {
    // Do this first... for reasons(?).
    let args = get_arguments();

    // Read in a `.env` file in the current directory, if it exists, and add the
    // `KEY=VALUE` entries to the application's `env`.
    dotenv().ok();
   
    // Get verbosity level for application
    let verbosity = env::var("VERBOSITY").unwrap_or("INFO".to_string());
    let level = verbosity.parse::<tracing::Level>().unwrap_or(tracing::Level::INFO);
    
    setup_tracing(level);

    args
}
