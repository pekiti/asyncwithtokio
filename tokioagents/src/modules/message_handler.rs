use tokio::sync::mpsc::{channel,Sender};
use tokio::sync::oneshot;

use failure::Fallible;

pub use crate::modules::message_generator::message_generator;

use crate::modules::message::Message;
use crate::modules::message_to_file::save_message;
use crate::modules::command::{Command, CommandResponse};

pub struct MessageHandler {
    ctrl: Sender<(Command, oneshot::Sender<CommandResponse>)> 
}

impl MessageHandler {

    pub fn spawn(filepath: &'static str) -> Fallible<MessageHandler> {
        let (tx,rx) = channel::<Message>(10);

        let (ctx,crx) = channel::<(Command, oneshot::Sender<CommandResponse>)>(10);

        tokio::spawn(message_generator(crx, tx));
        tokio::spawn(save_message(&filepath, rx));

        Ok(MessageHandler { ctrl: ctx })
    }

    pub async fn send_command(&mut self, cmd: Command) -> Fallible<CommandResponse> {
        let (rtx,rrx) = oneshot::channel::<CommandResponse>();
        self.ctrl.send((cmd, rtx)).await?; 

        Ok(rrx.await?)
    }

}
