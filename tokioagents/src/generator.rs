use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::timer::sleep;

#[derive(Debug)]
pub enum Message {
    Hello,
    Rust,
}

pub async fn message_generator(channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(100).await,
            Err(_) => {
                eprintln!("Error sending message");
                break;
            }
        }
    }
}
