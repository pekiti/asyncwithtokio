use tokio::sync::mpsc::channel;

mod agents;
use agents::generator_agent::{message_generator, Message};
use agents::receiver_agent::message_sink;

mod utils;
use utils::timer::sleep;

#[tokio::main]
async fn main() {
    let (tx, rc) = channel::<Message>(32);

    tokio::spawn(message_generator(tx));
    tokio::spawn(message_sink(rc));

    sleep(2000).await;
}
