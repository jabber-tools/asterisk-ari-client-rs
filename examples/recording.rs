use asterisk_ari_client_rs::apis::channels::ChannelsAPI;
use asterisk_ari_client_rs::apis::recordings::RecordingsAPI;
use asterisk_ari_client_rs::models::events::*;
use asterisk_ari_client_rs::{client::AriClient, errors::Result};
use env_logger;
use lazy_static::lazy_static;
use log::*;
use std::fs;
use std::io::Write;
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

fn stasis_start(event: StasisStart) {
    info!("stasis_start: {:#?}", event);
    tokio::spawn(async move {
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
    });
}

fn recording_started(event: RecordingStarted) {
    info!("recording_started: {:#?}", event);
}

fn recording_finished(event: RecordingFinished) {
    info!("recording_finished: {:#?}", event);

    tokio::spawn(async move {
        let recording_bytes = ARICLIENT
            .get_recording(&event.recording.name)
            .await
            .unwrap();

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!(
                "/tmp/{}.{}",
                event.recording.name, event.recording.format
            ))
            .unwrap();

        file.write_all(&recording_bytes).unwrap();
    });
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

    let (tx_recording_started, mut rx_recording_started) = mpsc::channel::<RecordingStarted>(1000);
    client.set_recording_started_sender(Some(tx_recording_started));

    let (tx_recording_finished, mut rx_recording_finished) =
        mpsc::channel::<RecordingFinished>(1000);
    client.set_recording_finished_sender(Some(tx_recording_finished));

    tokio::spawn(async move {
        if let Err(some_error) = client.ari_processing_loop(vec!["my-ast-app".into()]).await {
            error!("Error in ari_processing_loop {:?}", some_error);
        }
    });

    tokio::spawn(async move {
        loop {
            tokio::select! {
                event_opt = rx_stasis_start.recv() => {
                    if let Some(event) = event_opt {
                        stasis_start(event);
                    }
                }

                event_opt = rx_recording_started.recv() => {
                  if let Some(event) = event_opt {
                    recording_started(event);
                  }
                }

                event_opt = rx_recording_finished.recv() => {
                  if let Some(event) = event_opt {
                    recording_finished(event);
                  }
                }
            }
        }
    });

    sleep(Duration::from_millis(30000)).await;

    Ok(())
}
