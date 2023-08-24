use tokio::sync::mpsc::{channel, Receiver, Sender};

mod timer;
use timer::sleep;

mod generator;
use generator::{message_generator, Message};

async fn message_sink(mut channel: Receiver<Message>) {
    loop {
        match channel.recv().await {
            Some(Message::Hello) => println!("Hello"),
            Some(Message::Rust) => println!("Rust"),
            None => {
                eprintln!("Channel closed");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rc) = channel::<Message>(32);

    tokio::spawn(message_generator(tx));
    tokio::spawn(message_sink(rc));

    sleep(2000).await;
}
