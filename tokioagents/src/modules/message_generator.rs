use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use crate::utils::timer::sleep_for_1_second;

use crate::modules::message::Message;
use crate::modules::command::{ Command, CommandResponse, StopResponse, StatusResponse };


pub async fn message_generator(
    mut ctrl: Receiver<(Command, oneshot::Sender<CommandResponse>)>,
    channel: Sender<Message>,
) {
    loop {
        tokio::select! {
            msg = channel.send(Message::Hello) =>
                match msg  {
                    Ok(()) => sleep_for_1_second().await,
                    Err(_) => {
                        log::error!("Error sending message");
                        break;
                    }
                },
            ctl = ctrl.recv() => {
                match ctl {
                    Some((Command::Stop, rtx)) => {
                        rtx.send(CommandResponse::Stop(StopResponse::Stopped)).expect("unable to respond to ctrl message");
                        break;
                    },
                    Some((Command::Status, rtx)) => {
                        rtx.send(CommandResponse::Status(StatusResponse::Stable)).expect("unable to respond to ctrl message");
                    }
                    None => break // all senders have dropped
                }
            }
        }
    }
    log::info!("Message generator stopped");
}
