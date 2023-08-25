use std::fmt::Debug;
use tokio::fs::File;

use failure::Fallible;
use serde::Serialize;

use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::Receiver;

pub async fn save_message<T: Debug + Serialize>(
    filepath: &'static str,
    mut channel: Receiver<T>,
) -> Fallible<()> {
    log::info!("Open File: {}.bin", filepath);

    let mut file = File::create(format!("data/messages/{}.bin", filepath))
        .await
        .expect("cannot open file");

    while let Some(msg) = channel.recv().await {
        log::info!("File: {}.bin\nMessage: {:?}", filepath, msg);

        file.write(&bincode::serialize(&msg)?)
            .await
            .expect("cannot write to file");
    }

    Ok(())
}
