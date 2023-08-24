use tokio::sync::mpsc::{channel, Receiver, Sender};

async fn sleep(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

#[derive(Debug)]
enum Message {
    Hello,
    Rust,
}

async fn message_generator(channel: Sender<Message>) {
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
