use failure::Fallible;
use tokio::sync::mpsc::channel;
use tokio::sync::oneshot;

mod agents;
use agents::messenger_agent::{messenger_execute, Command, StatusResponse, Message};
use agents::receiver_agent::message_sink;

mod utils;
use utils::timer::sleep;

#[tokio::main]
async fn main() -> Fallible<()> {
    let (tx, rc) = channel::<Message>(32);
    let (ctx, crx) = channel::<Command>(32);

    tokio::spawn(messenger_execute(crx, tx));
    tokio::spawn(message_sink(rc));

    sleep(5).await;

    let (rtx, rrx) = oneshot::channel::<StatusResponse>();
    println!(">>>> Sending STATUS command <<<<");

    ctx.send(Command::Status(rtx)).await?;
    println!(">>>> Response: {:?} <<<<", rrx.await?);

    sleep(5).await;

    println!(">>>> Sending STOP command <<<<");
    ctx.send(Command::Stop).await?;

    //sleep(1000).await;

    Ok(())
}
