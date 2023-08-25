use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use tokio::sync::mpsc::{channel, Sender, Receiver};

use failure::Fallible;

use crate::modules::message_handler::MessageHandler;
use crate::modules::message_receiver_handler::{MessageReceiverHandler, MessageReceivers};
use crate::modules::command::{Command, StatusResponse};

pub struct MessageHandlerSpawner {
   ctrl_tx: Sender<Command> 
}

impl MessageHandlerSpawner {
    pub fn spawn(msgss: MessageReceiverHandler) -> Fallible<MessageHandlerSpawner> {
        let (ctx, crx) = channel::<Command>(10);

        tokio::spawn(agent_loop(crx, msgss));

        Ok(MessageHandlerSpawner {
           ctrl_tx: ctx 
        })
   }

    pub async fn send_command(&mut self, msg: Command) -> Fallible<()> {
        self.ctrl_tx.send(msg).await;

        Ok(())
    }
}

pub async fn agent_loop(mut ctrl: Receiver<Command>, mut msgss: MessageReceiverHandler) {
    
    let mut current_receivers: MsgRecRegistry = HashMap::new();

    loop {
        tokio::select! {
            msg = msgss.subjects_rx.recv() => {
                let receivers = msg.unwrap();
                process_command(&mut current_receivers, &receivers).await;
            },
            ctl = ctrl.recv() => {
                match ctl {
                    Some(Command::Stop) => break,
                    Some(Command::Status) => log::info!("Status: {:?}", StatusResponse::Stable),
                    None => break
                }
            }
        }
    }
    log::info!("MessageHandlerSpawner finished");
}

type MsgRecRegistry = HashMap<&'static str,MessageHandler>;

async fn process_command(mrr: &mut MsgRecRegistry, subjects: &MessageReceivers) {
    let active: HashSet<&str> = HashSet::from_iter(mrr.keys().copied());
    let desired: HashSet<&str> = HashSet::from_iter(subjects.iter().copied());

    let intersection = active.intersection(&desired).copied().collect();

    let to_spawn = desired.difference(&intersection);
    let to_stop = active.difference(&intersection);

    log::info!("to_quit {:?}", to_stop);
    
    for s in to_stop {
        log::info!("Stopping {}", s);
        let mut msgr = mrr.remove(s).unwrap();
        msgr.send_command(Command::Stop).await.expect("cannot send stop command");
    }

    log::info!("to_spawn {:?}", to_spawn);
    
    for s in to_spawn {
        log::info!("Spawning {}", s);
        mrr.insert(s, MessageHandler::spawn(s).expect("unable to spawn message handler"));
    }
}
