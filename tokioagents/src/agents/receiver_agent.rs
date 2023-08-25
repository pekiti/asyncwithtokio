use tokio::sync::mpsc::Receiver;

use crate::agents::messenger_agent::Message;

pub async fn message_sink(mut channel: Receiver<Message>) {
    loop {
        match channel.recv().await {
            Some(Message::Hello) => println!("Hello, Rust!"),
            None => {
                println!("[Receiver] >>>> Channel closed <<<<");
                break;
            }
        }
    }
}
