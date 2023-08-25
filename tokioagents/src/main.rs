use failure::Fallible;
use log::Level;
use simple_logger;

mod utils;
use utils::timer::sleep_for_2_seconds;

mod modules;
use modules::message_handler_spawner::MessageHandlerSpawner;
use modules::message_receiver_handler::MessageReceiverHandler;

#[tokio::main]
async fn main() -> Fallible<()> {
    simple_logger::init_with_level(Level::Info).unwrap();

    let msgss = MessageReceiverHandler::spawn()?;
    let _msgr = MessageHandlerSpawner::spawn(msgss)?;

    sleep_for_2_seconds().await; // print 20 messages

    log::info!("Exit program");
   
    Ok(())
}
