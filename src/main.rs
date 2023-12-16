#![allow(dead_code, unused)]
use std::time::Duration;

use log::Level;
use tokio::{io::AsyncReadExt, join, time};

// fibonacci function to find the nth number fibonacci
fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 1),
    }
}
// sleeper function sleep for 1 sec
async fn sleeper() {
    log::info!("Sleeping"); // log information message
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // sleep for 1 sec
    log::info!("Awake!") // log message
}

// reader function
async fn reader() {
    log::info!("Reading some beeg data"); // Log informatin message
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap(); // open the file
    let mut contents = vec![]; // create a mutable vector
    f.read_to_end(&mut contents).await.unwrap(); //  read the file content
    log::info!("Read beeg {} bytes", contents.len()); // print the file content length

    // call fibnacci method here
    // to done a heaivy task , creaded a spawing a new thread
    tokio::task::spawn_blocking(move || {
        fib(30);
        log::info!("Done computing fib(30");
    })
    .await
    .unwrap()
}

async fn do_something_fun() {
    log::info!("Doing something fun");
    tokio::time::sleep(Duration::from_secs(1)).await;
    log::info!("Fun was had");
}

async fn run() {
    // log::info!("Sleeping"); // Log message, this is information  log messae , it will print in console.
    // time::sleep(time::Duration::from_secs(1)).await; // this will sleep for a sec and execute it the go to the next operation
    // log::info!("Awake!"); // information log message

    // running syncronously
    // sleep().await;
    // reader().await;

    // join function in tokio module

    //    tokio::join!(
    //     sleeper(),
    //     reader(),
    //     reader(),
    //     reader(),
    //     reader(),
    //     reader(),
    //     reader(),
    //     reader(),

    //    );

    // adding fib  method(expensive method ) it's getting downgrade
    // tokio::join!(
    //     sleeper(),
    //     reader(),
    //     reader(),
    //     reader(),
    //     reader(),
    //     reader()

    // );
}

#[tokio::main]
async fn main() {
    // here we initialte simple_logger to bind with log
    simple_logger::init_with_level(Level::Info).unwrap();
    // // here we need runtime to execute async code. So we initiate a tokio runtime in main
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // // run() async function that will run by tokio run time
    // let future = run();
    // // block_on is a executor here to execute future
    // rt.block_on(future);

    // Modififcation after adding #[tokio::main]
    let start = std::time::Instant::now(); // capture instant timestmp
                                           // run() // futures are lazy , it will not execute untill call .await
    run().await; // executing run funciton
    let end = std::time::Instant::now(); // capture instant timestamp
    println!("Took {:?} seconds", end - start); // printing time diffencnce between start and end
}
