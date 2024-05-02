use crate::apis::{
    applications::ApplicationsAPI, channels::ChannelsAPI, recordings::RecordingsAPI,
};
use crate::errors::{Error, Result};
use crate::models::applications::Application;
use crate::models::channels::{Channel, Direction, Variable};
use crate::models::events::*;
use crate::models::playbacks::Playback;
use async_trait::async_trait;
use futures_util::SinkExt;
use lazy_static::lazy_static;
use log::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rand::Rng;
use reqwest::StatusCode;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
};
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message as WSMessage};
use url::Url;
use urlencoding::encode;

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

// items from traits can only be used if the trait is in scope
// this is brought in scope so that ws_stream.next() works!
use futures_util::StreamExt; // SinkExt needed for ws_stream.send(msg);

#[derive(Clone)]
pub struct AriClient {
    pub url: String,
    pub user: String,
    pub password: String,
    stasis_start_sender: Option<Sender<StasisStart>>,
    channel_dtmf_received_sender: Option<Sender<ChannelDtmfReceived>>,
    channel_hangup_request_sender: Option<Sender<ChannelHangupRequest>>,
    stasis_end_sender: Option<Sender<StasisEnd>>,
    channel_talking_finished_sender: Option<Sender<ChannelTalkingFinished>>,
    channel_talking_started_sender: Option<Sender<ChannelTalkingStarted>>,
    channel_destroyed_sender: Option<Sender<ChannelDestroyed>>,
    playback_started_sender: Option<Sender<PlaybackStarted>>,
    playback_finished_sender: Option<Sender<PlaybackFinished>>,
    channel_state_change_sender: Option<Sender<ChannelStateChange>>,
    channel_var_set_sender: Option<Sender<ChannelVarset>>,
    recording_started_sender: Option<Sender<RecordingStarted>>,
    recording_finished_sender: Option<Sender<RecordingFinished>>,
}

impl AriClient {
    pub fn new(url: String, user: String, password: String) -> Self {
        AriClient {
            url,
            user,
            password,
            stasis_start_sender: None,
            channel_dtmf_received_sender: None,
            channel_hangup_request_sender: None,
            stasis_end_sender: None,
            channel_talking_finished_sender: None,
            channel_talking_started_sender: None,
            channel_destroyed_sender: None,
            playback_started_sender: None,
            playback_finished_sender: None,
            channel_state_change_sender: None,
            channel_var_set_sender: None,
            recording_started_sender: None,
            recording_finished_sender: None,
        }
    }

    pub fn set_stasis_start_sender(&mut self, sender: Option<Sender<StasisStart>>) {
        self.stasis_start_sender = sender;
    }

    pub fn set_channel_dtmf_received_sender(
        &mut self,
        sender: Option<Sender<ChannelDtmfReceived>>,
    ) {
        self.channel_dtmf_received_sender = sender;
    }

    pub fn set_channel_hangup_request_sender(
        &mut self,
        sender: Option<Sender<ChannelHangupRequest>>,
    ) {
        self.channel_hangup_request_sender = sender;
    }

    pub fn set_stasis_end_sender(&mut self, sender: Option<Sender<StasisEnd>>) {
        self.stasis_end_sender = sender;
    }

    pub fn set_channel_talking_finished_sender(
        &mut self,
        sender: Option<Sender<ChannelTalkingFinished>>,
    ) {
        self.channel_talking_finished_sender = sender;
    }

    pub fn set_channel_talking_started_sender(
        &mut self,
        sender: Option<Sender<ChannelTalkingStarted>>,
    ) {
        self.channel_talking_started_sender = sender;
    }

    pub fn set_channel_destroyed_sender(&mut self, sender: Option<Sender<ChannelDestroyed>>) {
        self.channel_destroyed_sender = sender;
    }

    pub fn set_playback_started_sender(&mut self, sender: Option<Sender<PlaybackStarted>>) {
        self.playback_started_sender = sender;
    }

    pub fn set_playback_finished_sender(&mut self, sender: Option<Sender<PlaybackFinished>>) {
        self.playback_finished_sender = sender;
    }

    pub fn set_channel_state_change_sender(&mut self, sender: Option<Sender<ChannelStateChange>>) {
        self.channel_state_change_sender = sender;
    }

    pub fn set_channel_var_set_sender(&mut self, sender: Option<Sender<ChannelVarset>>) {
        self.channel_var_set_sender = sender;
    }

    pub fn set_recording_started_sender(&mut self, sender: Option<Sender<RecordingStarted>>) {
        self.recording_started_sender = sender;
    }

    pub fn set_recording_finished_sender(&mut self, sender: Option<Sender<RecordingFinished>>) {
        self.recording_finished_sender = sender;
    }

    /// connect to ARI signal stream websocket
    pub async fn ari_processing_loop(&self, asterisk_apps: Vec<String>) -> Result<()> {
        let ws_protocol = if self.url.starts_with("https://") {
            "wss"
        } else {
            "ws"
        };

        let url = Url::parse(&self.url)?;
        let hostname;
        if let Some(host) = url.host_str() {
            hostname = host;
        } else {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                Some("unable to parse hostname".into()),
            ));
        }

        let portno;
        if let Some(por) = url.port() {
            portno = por;
        } else {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                Some("unable to parse port".into()),
            ));
        }

        let app_str = asterisk_apps.join(",");

        // at the moment we are not supporting 'subscribeAll=true' option.
        // // will be added once/if needed
        let ws_url_str = format!(
            "{}://{}:{}/ari/events?app={}&api_key={}:{}",
            ws_protocol,
            hostname,
            portno,
            app_str,
            encode(&self.user),
            encode(&self.password)
        );

        let ws_url = Url::parse(&ws_url_str)?;

        debug!("connecting to ws_url: {}", ws_url);
        let (ws_stream, _) = connect_async(ws_url).await?;

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        debug!("websocket connected");

        let mut interval = interval(Duration::from_millis(5000));

        loop {
            tokio::select! {
                msg = ws_receiver.next() => {
                    match msg {
                        Some(msg) => {
                            let msg = msg?;
                            match msg {
                                    WSMessage::Close(close_frame) => {
                                        info!(
                                            "close message received, leaving the loop! {:#?}",
                                            close_frame
                                        );
                                        break;
                                    }
                                    WSMessage::Pong(_) => {}
                                    WSMessage::Ping(data) => {
                                        let _ = ws_sender.send(WSMessage::Pong(data)).await;
                                    }
                                    WSMessage::Text(string_msg) => {
                                        debug!(
                                            "asterisk signal event received: {:#?}",
                                            string_msg
                                        );
                                        let ari_event =
                                            serde_json::from_str::<AriEvent>(&string_msg);
                                        if let Err(deser_err) = ari_event {
                                            warn!(
                                                "error when deserializing ARI event: {:#?}. Event: {:#?}",
                                                deser_err, string_msg
                                            );
                                        } else {
                                            let ari_event = ari_event.unwrap();
                                            trace!("ari_event: {:#?}", ari_event);
                                            match ari_event {
                                                AriEvent::StasisStart(event) => {
                                                    if let Some(sender) = &self.stasis_start_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop StasisStart sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelDtmfReceived(event) => {
                                                    if let Some(sender) = &self.channel_dtmf_received_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelDtmfReceived sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelHangupRequest(event) => {
                                                    if let Some(sender) = &self.channel_hangup_request_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelHangupRequest sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::StasisEnd(event) => {
                                                    if let Some(sender) = &self.stasis_end_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop StasisEnd sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelTalkingFinished(event) => {
                                                    if let Some(sender) = &self.channel_talking_finished_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelTalkingFinished sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelTalkingStarted(event) => {
                                                    if let Some(sender) = &self.channel_talking_started_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelTalkingStarted sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelDestroyed(event) => {
                                                    if let Some(sender) = &self.channel_destroyed_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelDestroyed sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::PlaybackStarted(event) => {
                                                    if let Some(sender) = &self.playback_started_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop PlaybackStarted sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::PlaybackFinished(event) => {
                                                    if let Some(sender) = &self.playback_finished_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop PlaybackFinished sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelStateChange(event) => {
                                                    if let Some(sender) = &self.channel_state_change_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelStateChange sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::ChannelVarset(event) => {
                                                    if let Some(sender) = &self.channel_var_set_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop ChannelVarset sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::RecordingStarted(event) => {
                                                    if let Some(sender) = &self.recording_started_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop RecordingStarted sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                                AriEvent::RecordingFinished(event) => {
                                                    if let Some(sender) = &self.recording_finished_sender {
                                                        if let Err(send_err) = sender.send(event.clone()).await {
                                                            error!("ari_processing_loop RecordingFinished sending error {:?}: ", send_err);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        warn!(
                                            "unknown websocket message received: {:#?}",
                                            msg
                                        );
                                    }
                                }
                        }
                        None => break,
                    }
                }
                _ = interval.tick() => {
                    // every 5 seconds we are sending ping to keep connection alive
                    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
                    let random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();
                    let _ = ws_sender.send(WSMessage::Ping(random_bytes)).await;
                    debug!("ari connection ping sent");
                }
            }
        }

        Ok(())
    }

    #[allow(deprecated)]
    fn get_auth_header(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(format!("{}:{}", self.user, self.password))
        )
    }

    fn get_common_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&self.get_auth_header())?,
        );

        Ok(headers)
    }
}

macro_rules! eval_status_code {
    ($status_real:ident, $status_expected:expr, $body_str:expr) => {
        if $status_real != $status_expected {
            return if let Some(some_body) = $body_str {
                Err(Error::new($status_real, Some(some_body)))
            } else {
                Err(Error::new($status_real, None))
            };
        }
    };
}

#[async_trait]
impl ApplicationsAPI for AriClient {
    /// Filter application events types.
    #[allow(unused_variables)]
    async fn filter(
        &self,
        application_name: &str,
        filter: Option<serde_json::Value>,
    ) -> Result<String> {
        Err(Error::new(StatusCode::NOT_IMPLEMENTED, None))
    }

    /// Get details of an application.
    async fn get(&self, application_name: &str) -> Result<Application> {
        let resp = HTTP_CLIENT
            .get(format!("{}/applications/{}", self.url, application_name))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::OK, Some(body_str));
        Ok(serde_json::from_str(&body_str)?)
    }

    /// List all applications.
    async fn list(&self) -> Result<Vec<Application>> {
        let resp = HTTP_CLIENT
            .get(format!("{}/applications", self.url))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::OK, Some(body_str));
        Ok(serde_json::from_str(&body_str)?)
    }

    /// Subscribe an application to a event source.
    #[allow(unused_variables)]
    async fn subscribe(&self, application_name: &str, event_source: Vec<String>) -> Result<String> {
        Err(Error::new(StatusCode::NOT_IMPLEMENTED, None))
    }

    /// Unsubscribe an application from an event source.
    #[allow(unused_variables)]
    async fn unsubscribe(
        &self,
        application_name: &str,
        event_source: Vec<String>,
    ) -> Result<String> {
        Err(Error::new(StatusCode::NOT_IMPLEMENTED, None))
    }
}

#[async_trait]
impl ChannelsAPI for AriClient {
    async fn answer(&self, channel_id: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .post(format!("{}/channels/{}/answer", self.url, channel_id))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }

    async fn play(
        &self,
        channel_id: &str,
        media: &str,
        _playback_id: Option<String>,
        _lang: Option<String>,
        _offsetms: Option<usize>,
        _skipms: Option<usize>,
    ) -> Result<Playback> {
        // so far we are not supporting optional parameters
        let req_body = format!(
            r#"
            {{
                "channelId": "{_channel_id_}",
                "media": "{_media_}"
            }}
            "#,
            _channel_id_ = channel_id,
            _media_ = media,
        );

        let resp = HTTP_CLIENT
            .post(format!("{}/channels/{}/play", self.url, channel_id))
            .headers(self.get_common_headers()?)
            .body(req_body)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::CREATED, Some(body_str));
        Ok(serde_json::from_str(&body_str)?)
    }

    async fn stop_play(&self, playback_id: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .delete(format!("{}/playbacks/{}", self.url, playback_id))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }

    async fn get_variable(&self, channel_id: &str, var_name: &str) -> Result<String> {
        let resp = HTTP_CLIENT
            .get(format!(
                "{}/channels/{}/variable?variable={}",
                self.url, channel_id, var_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::OK, Some(body_str));

        let variable = serde_json::from_str::<Variable>(&body_str)?;
        Ok(variable.value)
    }

    async fn set_variable(&self, channel_id: &str, var_name: &str, var_value: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .post(format!(
                "{}/channels/{}/variable?variable={}&value={}",
                self.url, channel_id, var_name, var_value
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;

        eval_status_code!(status, StatusCode::NO_CONTENT, Some(body_str));
        Ok(())
    }

    async fn hangup(&self, channel_id: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .delete(format!("{}/channels/{}", self.url, channel_id))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::NO_CONTENT, Some(body_str));
        Ok(())
    }

    async fn continue_in_dialplan(&self, channel_id: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .post(format!("{}/channels/{}/continue", self.url, channel_id))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::NO_CONTENT, Some(body_str));
        Ok(())
    }

    async fn snoop(
        &self,
        channel_id: &str,
        app: &str,
        spy: Option<Direction>,
        whisper: Option<Direction>,
    ) -> Result<Channel> {
        let req_body = format!(
            r#"
            {{
                "app": "{_app_name_}",
                "spy": "{_spy_}",
                "whisper": "{_whisper_}"
            }}
            "#,
            _app_name_ = app,
            _spy_ = spy.unwrap_or_default(),
            _whisper_ = whisper.unwrap_or_default()
        );

        let req = HTTP_CLIENT
            .post(format!("{}/channels/{}/snoop", self.url, channel_id))
            .headers(self.get_common_headers()?)
            .body(req_body.clone());
        trace!("req: {req:#?}");
        trace!("req body: {}", req_body);
        trace!("url: {:#?}", self.url);

        let resp = req.send().await?;

        trace!("response: {:#?}", resp);

        let status = resp.status();
        trace!("status: {:#?}", status);

        let body_str = resp.text().await?;
        trace!("text: {:#?}", body_str);

        eval_status_code!(status, StatusCode::OK, Some(body_str));

        let res_chan = serde_json::from_str(&body_str)?;
        Ok(res_chan)
    }

    async fn record(
        &self,
        channel_id: &str,
        filepath: Option<&str>,
        audio_format: Option<&str>,
        terminate_on: Option<&str>,
        max_duration: Option<usize>,
        max_silence: Option<usize>,
        if_exists: Option<&str>,
        beep: Option<bool>,
    ) -> Result<()> {
        let req_body = format!(
            r#"
            {{
                "name": "{_filepath_}",
                "format": "{_audio_format_}",
                "terminateOn": "{_terminate_on_}",
                "maxDuration": {_max_duration_},
                "maxSilence": {_max_silence_},
                "ifExists": "{_if_exists_}",
                "beep": {_beep_}
            }}
            "#,
            _filepath_ = filepath.unwrap_or(channel_id),
            _audio_format_ = audio_format.unwrap_or("wav"),
            _terminate_on_ = terminate_on.unwrap_or("none"),
            _max_duration_ = max_duration.unwrap_or(0),
            _max_silence_ = max_silence.unwrap_or(0),
            _if_exists_ = if_exists.unwrap_or("fail"),
            _beep_ = beep.unwrap_or(false),
        );
        let resp = HTTP_CLIENT
            .post(format!("{}/channels/{}/record", self.url, channel_id))
            .headers(self.get_common_headers()?)
            .body(req_body)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::CREATED, Some(body_str));
        Ok(())
    }
}

#[async_trait]
impl RecordingsAPI for AriClient {
    async fn get_recording(&self, recording_name: &str) -> Result<Vec<u8>> {
        let recording_name = utf8_percent_encode(recording_name, NON_ALPHANUMERIC);
        let resp = HTTP_CLIENT
            .get(format!(
                "{}/recordings/stored/{}/file",
                self.url, recording_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;
        let status = resp.status();
        let body_bytes = resp.bytes().await?;

        eval_status_code!(status, StatusCode::OK, Some(format!("{body_bytes:#?}")));
        Ok(body_bytes.to_vec())
    }
    async fn stop_recording(&self, recording_name: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .post(format!(
                "{}/recordings/live/{}/stop",
                self.url, recording_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        let body_str = resp.text().await?;
        eval_status_code!(status, StatusCode::NO_CONTENT, Some(body_str));
        Ok(())
    }

    async fn pause_recording(&self, recording_name: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .post(format!(
                "{}/recordings/live/{}/pause",
                self.url, recording_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }

    async fn unpause_recording(&self, recording_name: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .delete(format!(
                "{}/recordings/live/{}/pause",
                self.url, recording_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }

    async fn mute_recording(&self, recording_name: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .post(format!(
                "{}/recordings/live/{}/mute",
                self.url, recording_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }

    async fn unmute_recording(&self, recording_name: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .delete(format!(
                "{}/recordings/live/{}/mute",
                self.url, recording_name
            ))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }

    async fn delete_recording(&self, recording_name: &str) -> Result<()> {
        let resp = HTTP_CLIENT
            .delete(format!("{}/recordings/live/{}", self.url, recording_name))
            .headers(self.get_common_headers()?)
            .send()
            .await?;

        let status = resp.status();
        eval_status_code!(status, StatusCode::NO_CONTENT, None);
        Ok(())
    }
}
