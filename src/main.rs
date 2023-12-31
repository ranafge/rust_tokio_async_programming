#![allow(dead_code, unused)]
use std::time::Duration;
use tokio::{self, io::AsyncWriteExt, process, signal};
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

struct MyStruct {
    field: i32
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

    use tokio::sync;
    // thread safe with mutiability impl(Send marker trait)
    let lock = std::sync::Arc::new(
        sync::Mutex::new(MyStruct{field: 0})
    );

    // one of the way of creating reference count of lock
    let lock_a = lock.clone();
    let lock_b = lock.clone();
    // spawning new thread and modifing struct field value
    let a = tokio::spawn(async move {
        let mut val = lock_a.lock().await;
        val.field = 1
    });
    // spawning new thread and modifing struct field value
    let b  = tokio::spawn(async move  {
        let mut val = lock_b.lock().await;
        val.field = 2;
    });
    // execute the a and b joinhandler 
    tokio::join!(a, b).1.unwrap();
    // printing the modified struct field
    let val = lock.lock().await;
    println!("Value is: {}", val.field);

    // mpsc channel with capacity 
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let tx1= tx.clone();

    tokio::spawn(async move {
        for i in 0..=20{
            tx.send(i).await.unwrap();
        }
    });
    tokio::spawn(async move {
        for i in 0..=20{
            tx1.clone().send(i).await.unwrap();
        }
    });

    // received the sending value

    while let Some(value) = rx.recv().await {
        println!("GOT THE VALUE {}", value);
    }


    // tokio net
    use tokio::net;

    // let host = "localhost::8080";
    // // start up tcp server
    // // FACING ERROR IN THIS SECTION.
    // let srv = tokio::net::TcpListener::bind(host).await.unwrap();

    // loop {
    //     // accept new connection
    //     let (mut sock, _) = srv.accept().await.unwrap();
    //     // Spawn a new task to handle the connection
    //     tokio::spawn(async move {
    //         let mut buf = [0; 1024];
    //         let n  = sock.read(&mut buf).await.unwrap();
    //         sock.write_all(&buf[0..n]).await.unwrap();
    //         let data  = std::str::from_utf8(&buf[0..n]).unwrap();
    //         println!("ECHOED: {:?}", data);
    //         sock.shutdown().await.unwrap();


    //     });
    //     // tokio task

    //     let task_a = tokio::task::spawn_blocking(|| {
    //         println!("Starting fib(30) computation...");
    //         let res = fib(30);
    //         println!("fib(30 = {}", res);
    //     });
    //     tokio::join!(task_a).0.unwrap();


    // }

        let res = reading_file().await;

        println!("Result of file {:?}", res);
        let res_std_out = process_manage().await;
        println!("The out put of process_manage fun {:?}", res_std_out);
        tokio_signal().await;
        tokio_time().await;
        tokio_time_out().await;

}



async fn reading_file() ->Result<(), Box<dyn std::error::Error>> {
    let mut file_ = tokio::fs::File::open("beeg.csv").await?;
    let mut contents = String::new();
    file_.read_to_string(&mut contents).await?;
    println!("File contents: {}", contents);

    let mut outfile = tokio::fs::File::create("out.txt").await?;
    outfile.write_all(contents.as_bytes()).await?;
    Ok(())
}

async fn process_manage() ->Result<(), Box<dyn std::error::Error>> {
    let mut cmd = process::Command::new("sort");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stdin(std::process::Stdio::piped());
    let mut child = cmd.spawn()?;
    let animal = &["dog", "bird", "frog", "cat", "fish"];
    let mut stdin = child.stdin.take().expect("child did not have a handle to stdin");
    stdin.write(animal.join("\n").as_bytes()).await.expect("could not write to stdin");
    drop(stdin);
    let op = child.wait_with_output().await?;
    println!("sorted: \n\n{}", std::str::from_utf8(&op.stdout)?);

    Ok(())
}

async fn tokio_signal() ->Result<(), Box<dyn std::error::Error>> {
    println!("Waiting for ctrl-c");
    signal::ctrl_c().await?;
    println!("received ctrl-c event");
    Ok(())
}

async fn tokio_time() ->Result<(), Box<dyn std::error::Error>> {

    let duration = tokio::time::Duration::from_secs(1);
    let mut when = tokio::time::interval(duration);
    when.tick().await;
    println!("Tick 1");
    when.tick().await;
    println!("Tick 2");
    when.tick().await;
    println!("Tick 3");
    Ok(())
}

async fn tokio_time_out() -> Result<(), Box<dyn std::error::Error>>{
    simple_logger::init_with_level(log::Level::Info)?;
    if let Err(_) = tokio::time::timeout (
        tokio::time::Duration::from_secs(2), sleepy(),
    ).await {
        log::info!("Sleepy time out");
    };
    Ok(())
}

async fn sleepy() {
    println!("starting sleepy");
    tokio::time::sleep(Duration::from_secs(10)).await;
    log::info!("Ending sleepy");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_do_something() {
        assert_eq!(3,3);
    }
}
























