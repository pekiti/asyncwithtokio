use rand::prelude::*;
use tokio::sync::mpsc::{channel, Receiver, Sender}; // TODO: migrate to rand 0.8.5

use failure::Fallible;

use crate::utils::timer::sleep_for_1_second;
use crate::modules::command::{Command, StatusResponse};

const LIST_OF_RECEIVERS: &'static [&'static str] = &[
    "receiver_a",
    "receiver_b",
    "receiver_c",
    "receiver_d",
    "receiver_e",
    "receiver_f",
    "receiver_g",
    "receiver_h",
];

pub struct MessageReceiverHandler {
    ctrl: Sender<Command>,
    subjects: Vec<String>,
    pub subjects_rx: Receiver<MessageReceivers>,
}

impl MessageReceiverHandler {
    pub fn spawn() -> Fallible<MessageReceiverHandler> {
        let (tx, rx) = channel::<MessageReceivers>(10);

        let (ctx, crx) = channel::<Command>(10);

        tokio::spawn(agent_loop(crx, tx));

        Ok(MessageReceiverHandler {
            subjects: Vec::new(),
            ctrl: ctx,
            subjects_rx: rx,
        })
    }

    pub async fn send_ctrl_msg(&mut self, msg: Command) -> Fallible<()> {
        self.ctrl.send(msg).await?;

        Ok(())
    }
}

pub type MessageReceivers = Vec<&'static str>;

fn build_random_list_of_receivers() -> MessageReceivers {
    let mut receivers: Vec<&'static str> = LIST_OF_RECEIVERS.into();

    let rng = &mut thread_rng();
    receivers.shuffle(rng);

    receivers
        .into_iter()
        .take(rng.gen_range(1, LIST_OF_RECEIVERS.len()))
        .collect()
}

// The agent loop
pub async fn agent_loop(mut ctrl: Receiver<Command>, channel: Sender<MessageReceivers>) {
    loop {
        let list_of_receivers = build_random_list_of_receivers();

        tokio::select! {
            msg = channel.send(list_of_receivers) =>
                match msg  {
                    Ok(()) => sleep_for_1_second().await,
                    Err(_) => {
                        log::error!("Error sending message");
                        break;
                    }
                },
            ctl = ctrl.recv() => {
                match ctl {
                    Some(Command::Stop) => {
                        break;
                    },
                    Some(Command::Status) => {
                        log::info!("Status: {:?}", StatusResponse::Stable);
                    },
                    None => break
                }
            }
        }
    }
    log::info!("Message subject scanner stopped");
}
