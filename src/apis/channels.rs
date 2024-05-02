use crate::errors::Result;
use crate::models::channels::{Channel, Direction};
use crate::models::playbacks::Playback;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelsAPI {
    /// Answer a channel.
    async fn answer(&self, channel_id: &str) -> Result<()>;
    /// Play media to a channel
    async fn play(
        &self,
        channel_id: &str,
        media: &str,
        _playback_id: Option<String>,
        _lang: Option<String>,
        _offsetms: Option<usize>,
        _skipms: Option<usize>,
    ) -> Result<Playback>;
    /// Stop playing particular playback
    async fn stop_play(&self, playback_id: &str) -> Result<()>;

    /// Get the value of a channel variable
    async fn get_variable(&self, channel_id: &str, var_name: &str) -> Result<String>;

    /// Set the value of a channel variable
    async fn set_variable(&self, channel_id: &str, var_name: &str, var_value: &str) -> Result<()>;

    /// Hang up the channel
    async fn hangup(&self, channel_id: &str) -> Result<()>;

    /// Exit application; continue execution in the dialplan
    async fn continue_in_dialplan(&self, channel_id: &str) -> Result<()>;

    /// Create a new channel to snoop (spy/whisper) on a specific channel
    async fn snoop(
        &self,
        channel_id: &str,
        app: &str,
        spy: Option<Direction>,
        whisper: Option<Direction>,
    ) -> Result<Channel>;

    /// Record audio from a channel. Default filepath: /var/spool/asterisk/recording/channel_id.wav
    #[allow(clippy::too_many_arguments)]
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
    ) -> Result<()>;
}
