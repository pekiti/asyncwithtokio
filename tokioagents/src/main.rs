use tokio::sync::mpsc::channel;

mod timer;
use timer::sleep;

mod generator;
use generator::{message_generator as messenger_send, Message};

mod sink;
use sink::message_sink;

#[tokio::main]
async fn main() {
    let (tx, rc) = channel::<Message>(32);

    tokio::spawn(messenger_send(tx));
    tokio::spawn(message_sink(rc));

    sleep(2000).await;
}
