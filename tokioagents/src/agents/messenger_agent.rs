use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

// #[non_exhaustive]  allows us adding more variants later.
// This means that the enum can’t be matched exhaustively and we’ll always need to add a wildcard match arm (_ => {}).
#[non_exhaustive]
pub enum Message {
    Hello,
    // Rust,
}
#[derive(Debug)]
pub enum StatusResponse {
    Healthy,
    Unhealthy,
}

pub enum Command {
    Stop,
    Status(oneshot::Sender<StatusResponse>),
}

pub async fn messenger_execute(mut command: Receiver<Command>, channel: Sender<Message>) {
    loop {
        tokio::select! {
            msg = channel.send(Message::Hello) => {
                match msg {
                    Ok(_) => {},
                    Err(e) => eprintln!(">>>> Error sending message: {} <<<<", e),
                }
            },
            ctl = command.recv() => {
                match ctl {
                    Some(Command::Stop) => {
                        println!(">>>> Received STOP command <<<<");
                        break;
                    },
                    Some(Command::Status(rtx)) => {
                        println!(">>>> Received STATUS command <<<<");
                        rtx.send(StatusResponse::Healthy).unwrap()
                    },
                    None => {
                        println!("[Messenger]  >>>> Channel closed <<<<");
                        break;
                    }
                }
            }
        }
    }
}
