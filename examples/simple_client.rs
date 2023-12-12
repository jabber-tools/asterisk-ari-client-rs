use asterisk_ari_client_rs::apis::applications::ApplicationsAPI;
use asterisk_ari_client_rs::apis::channels::ChannelsAPI;
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

fn stasis_start(event: StasisStart) {
    tokio::spawn(async move {
        info!("stasis_start: {:#?}", event);
        debug!("Answering channel {} now!", &event.channel.id);
        ARICLIENT.answer(&event.channel.id).await.unwrap();
        debug!("Channel {} answered!", &event.channel.id);
    });
}

fn channel_dtmf_received(event: ChannelDtmfReceived) {
    info!("channel_dtmf_received: {:#?}", event);
}

fn channel_hangup_request(event: ChannelHangupRequest) {
    info!("channel_hangup_request: {:#?}", event);
}

fn stasis_end(event: StasisEnd) {
    info!("stasis_end: {:#?}", event);
}

fn channel_talking_finished(event: ChannelTalkingFinished) {
    info!("channel_talking_finished: {:#?}", event);
}

fn channel_talking_started(event: ChannelTalkingStarted) {
    info!("channel_talking_started: {:#?}", event);
}

fn channel_destroyed(event: ChannelDestroyed) {
    info!("channel_destroyed: {:#?}", event);
}

fn playback_started(event: PlaybackStarted) {
    info!("playback_started: {:#?}", event);
}

fn playback_finished(event: PlaybackFinished) {
    info!("playback_finished: {:#?}", event);
}

fn channel_state_change(event: ChannelStateChange) {
    info!("channel_state_change: {:#?}", event);
}

fn channel_var_set(event: ChannelVarset) {
    info!("channel_var_set: {:#?}", event);
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut client = AriClient::new(
        "http://localhost:8088/ari".into(),
        "asterisk".into(),
        "asterisk".into(),
    );
    let resp = client.list().await?;
    debug!("asterisk registered apps: {:#?}", resp);

    // we could potentially retrieve application details like this:
    // let resp2 = client.get("my-ast-app").await?;
    // debug!("my app is {:#?}", resp2);

    let (tx_stasis_start, mut rx_stasis_start) = mpsc::channel::<StasisStart>(1000);
    client.set_stasis_start_sender(Some(tx_stasis_start));

    let (tx_channel_dtmf_received, mut rx_channel_dtmf_received) =
        mpsc::channel::<ChannelDtmfReceived>(1000);
    client.set_channel_dtmf_received_sender(Some(tx_channel_dtmf_received));

    let (tx_channel_hangup_request, mut rx_channel_hangup_request) =
        mpsc::channel::<ChannelHangupRequest>(1000);
    client.set_channel_hangup_request_sender(Some(tx_channel_hangup_request));

    let (tx_stasis_end, mut rx_stasis_end) = mpsc::channel::<StasisEnd>(1000);
    client.set_stasis_end_sender(Some(tx_stasis_end));

    let (tx_channel_talking_finished, mut rx_channel_talking_finished) =
        mpsc::channel::<ChannelTalkingFinished>(1000);
    client.set_channel_talking_finished_sender(Some(tx_channel_talking_finished));

    let (tx_channel_talking_started, mut rx_channel_talking_started) =
        mpsc::channel::<ChannelTalkingStarted>(1000);
    client.set_channel_talking_started_sender(Some(tx_channel_talking_started));

    let (tx_channel_destroyed, mut rx_channel_destroyed) = mpsc::channel::<ChannelDestroyed>(1000);
    client.set_channel_destroyed_sender(Some(tx_channel_destroyed));

    let (tx_playback_started, mut rx_playback_started) = mpsc::channel::<PlaybackStarted>(1000);
    client.set_playback_started_sender(Some(tx_playback_started));

    let (tx_playback_finished, mut rx_playback_finished) = mpsc::channel::<PlaybackFinished>(1000);
    client.set_playback_finished_sender(Some(tx_playback_finished));

    let (tx_channel_state_change, mut rx_channel_state_change) =
        mpsc::channel::<ChannelStateChange>(1000);
    client.set_channel_state_change_sender(Some(tx_channel_state_change));

    let (tx_channel_var_set, mut rx_channel_var_set) = mpsc::channel::<ChannelVarset>(1000);
    client.set_channel_var_set_sender(Some(tx_channel_var_set));

    tokio::spawn(async move {
        if let Err(some_error) = client
            .ari_processing_loop(vec!["my-ast-app".into()])
            .await
        {
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
                event_opt = rx_channel_dtmf_received.recv() => {
                    if let Some(event) = event_opt {
                        channel_dtmf_received(event);
                    }
                }

                event_opt = rx_channel_hangup_request.recv() => {
                    if let Some(event) = event_opt {
                        channel_hangup_request(event);
                    }
                }

                event_opt = rx_stasis_end.recv() => {
                    if let Some(event) = event_opt {
                        stasis_end(event);
                    }
                }

                event_opt = rx_channel_talking_finished.recv()	 => {
                    if let Some(event) = event_opt {
                        channel_talking_finished(event);
                    }
                }

                event_opt = rx_channel_talking_started.recv() => {
                    if let Some(event) = event_opt {
                        channel_talking_started(event);
                    }
                }

                event_opt = rx_channel_destroyed.recv() => {
                    if let Some(event) = event_opt {
                        channel_destroyed(event);
                    }
                }
                event_opt = rx_playback_started.recv() => {
                   if let Some(event) = event_opt {
                    playback_started(event);
                   }
                }

                event_opt = rx_playback_finished.recv() => {
                  if let Some(event) = event_opt {
                    playback_finished(event);
                  }
                }

                event_opt = rx_channel_state_change.recv() => {
                  if let Some(event) = event_opt {
                    channel_state_change(event);
                  }
                }

                event_opt = rx_channel_var_set.recv() => {
                  if let Some(event) = event_opt {
                    channel_var_set(event);
                  }
                }
            }
        }
    });

    sleep(Duration::from_millis(100000)).await;

    Ok(())
}
