use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use failure::Fallible;

use tokio::sync::mpsc::Receiver;

use serde::{Deserialize, Serialize};

pub async fn publish_message<T: Debug + Serialize>(mut channel: Receiver<T>) -> Fallible<()> {
    // open file
    let mut file = File::create("output.bin")?;

    while let Some(msg) = channel.recv().await {
        println!("Writing to file:  {:?}", msg);
        file.write(&bincode::serialize(&msg)?)?;
    }

    Ok(())
}
