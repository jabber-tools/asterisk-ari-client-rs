use asterisk_ari_client_rs::apis::channels::ChannelsAPI;
use asterisk_ari_client_rs::apis::recordings::RecordingsAPI;
use asterisk_ari_client_rs::models::events::*;
use asterisk_ari_client_rs::{client::AriClient, errors::Result};
use env_logger;
use lazy_static::lazy_static;
use log::*;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{self, sync::mpsc};

lazy_static! {
    pub static ref ARICLIENT: AriClient = AriClient::new(
        "http://localhost:8088/ari".to_owned(),
        "asterisk".into(),
        "asterisk".into(),
    );
}

async fn stasis_start(event: StasisStart) {
    info!("stasis_start: {:#?}", event);
    debug!("Answering channel {} now!", &event.channel.id);
    ARICLIENT.answer(&event.channel.id).await.unwrap();
    debug!("Channel {} answered!", &event.channel.id);

    // start the recording
    ARICLIENT
        .record(&event.channel.id, None, None, None, None, None, None, None)
        .await
        .unwrap();

    // pause recording after 2 secs
    sleep(Duration::from_millis(2000)).await;
    ARICLIENT.pause_recording(&event.channel.id).await.unwrap();

    // unpause recording after additional 2 secs
    sleep(Duration::from_millis(2000)).await;
    ARICLIENT
        .unpause_recording(&event.channel.id)
        .await
        .unwrap();

    // stop recording after 2 secs
    sleep(Duration::from_millis(2000)).await;
    ARICLIENT.stop_recording(&event.channel.id).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut client = AriClient::new(
        "http://localhost:8088/ari".into(),
        "asterisk".into(),
        "asterisk".into(),
    );

    let (tx_stasis_start, mut rx_stasis_start) = mpsc::channel::<StasisStart>(1000);
    client.set_stasis_start_sender(Some(tx_stasis_start));

    tokio::spawn(async move {
        if let Err(some_error) = client.ari_processing_loop(vec!["my-ast-app".into()]).await {
            error!("Error in ari_processing_loop {:?}", some_error);
        }
    });

    if let Some(event) = rx_stasis_start.recv().await {
        stasis_start(event).await;
    }

    sleep(Duration::from_millis(30000)).await;

    Ok(())
}
