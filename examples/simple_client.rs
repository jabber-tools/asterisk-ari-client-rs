use asterisk_ari_client_rs::apis::applications::ApplicationsAPI;
use asterisk_ari_client_rs::apis::channels::ChannelsAPI;
use asterisk_ari_client_rs::models::events::*;
use asterisk_ari_client_rs::{client::AriClient, client::AriClientEventHandlers, errors::Result};
use env_logger;
use lazy_static::lazy_static;
use log::*;
use std::boxed::Box;
use tokio;

lazy_static! {
    pub static ref ARICLIENT: AriClient = AriClient::new(
        "http://localhost:8088/ari".to_owned(),
        "<<user>>".into(),
        "<<password>>".into(),
    );
}

struct EvtHandlers;

impl AriClientEventHandlers for EvtHandlers {
    fn stasis_start(&self, event: StasisStart) {
        tokio::spawn(async move {
            info!("stasis_start: {:#?}", event);
            debug!("Answering channel {} now!", &event.channel.id);
            ARICLIENT.answer(&event.channel.id).await.unwrap();
            debug!("Channel {} answered!", &event.channel.id);
        });
    }

    fn channel_dtmf_received(&self, event: ChannelDtmfReceived) {
        info!("channel_dtmf_received: {:#?}", event);
    }

    fn channel_hangup_request(&self, event: ChannelHangupRequest) {
        info!("channel_hangup_request: {:#?}", event);
    }

    fn stasis_end(&self, event: StasisEnd) {
        info!("stasis_end: {:#?}", event);
    }

    fn channel_talking_finished(&self, event: ChannelTalkingFinished) {
        info!("channel_talking_finished: {:#?}", event);
    }

    fn channel_talking_started(&self, event: ChannelTalkingStarted) {
        info!("channel_talking_started: {:#?}", event);
    }

    fn channel_destroyed(&self, event: ChannelDestroyed) {
        info!("channel_destroyed: {:#?}", event);
    }

    fn playback_started(&self, event: PlaybackStarted) {
        info!("playback_started: {:#?}", event);
    }

    fn playback_finished(&self, event: PlaybackFinished) {
        info!("playback_finished: {:#?}", event);
    }

    fn channel_state_change(&self, event: ChannelStateChange) {
        info!("channel_state_change: {:#?}", event);
    }

    fn channel_var_set(&self, event: ChannelVarset) {
        info!("channel_var_set: {:#?}", event);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = AriClient::new(
        "http://localhost:8088/ari".into(),
        "<<user>>".into(),
        "<<password>>".into(),
    );
    let resp = client.list().await?;

    debug!("asterisk registered apps: {:#?}", resp);

    let evt_handlers = EvtHandlers;

    client
        .ari_processing_loop(
            vec!["<<asterisk-app-name>>".into()],
            Some(Box::new(evt_handlers)),
        )
        .await?;

    debug!("websocket connected!");

    // no when we connected our app we can list it
    let resp2 = client.get("<<asterisk-app-name>>").await?;
    debug!("my app is {:#?}", resp2);

    Ok(())
}
