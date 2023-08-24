use tokio::sync::mpsc::Sender;

use crate::timer::sleep;

pub enum Message {
    Hello,
    // Rust,
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
