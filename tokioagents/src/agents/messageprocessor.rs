use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Rust,
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

pub async fn execute_command(mut command: Receiver<Command>, channel: Sender<Message>) {
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
