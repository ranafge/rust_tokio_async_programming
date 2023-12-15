use log::Level;
use tokio::time;

async fn run() {
    log::info!("Sleeping"); // Log message, this is information  log messae , it will print in console.
    time::sleep(time::Duration::from_secs(1)).await; // this will sleep for a sec and execute it the go to the next operation 
    log::info!("Awake!"); // information log message
}


fn main() {
    // here we initialte simple_logger to bind with log
    simple_logger::init_with_level(Level::Info).unwrap();
    // here we need runtime to execute async code. So we initiate a tokio runtime in main 
    let rt = tokio::runtime::Runtime::new().unwrap();
    // run() async function that will run by tokio run time
    let future = run();
    // block_on is a executor here to execute future
    rt.block_on(future);

}
