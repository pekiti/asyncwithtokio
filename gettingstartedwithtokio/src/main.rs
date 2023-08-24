use log::Level;
use tokio::io::AsyncReadExt;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    log::info!("Sleeping ...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading some data...");
    let mut f = tokio::fs::File::open("Cargo.toml").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read {} bytes", contents.len());

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(42)");
        fib(42);
        log::info!("Done computing fib(42)");
    })
    .await
    .unwrap();
}

async fn run() {
    tokio::join!(
        sleeper(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
    );
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    // let rt = tokio::runtime::Runtime::new().unwrap();     // [*] replaced by #[tokio::main]
    // let future = run();                                   // [*] replaced by #[tokio::main]

    let start = std::time::Instant::now();
    // rt.block_on(future);                                  // [*] replaced by #[tokio::main]
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start);
}
