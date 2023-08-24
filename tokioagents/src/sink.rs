use tokio::sync::mpsc::Receiver;

use crate::generator::Message;

pub async fn message_sink(mut channel: Receiver<Message>) {
    loop {
        match channel.recv().await {
            Some(Message::Hello) => println!("Hello"),
            // Some(Message::Rust) => println!("Rust"),
            None => {
                eprintln!("Channel closed");
                break;
            }
        }
    }
}
