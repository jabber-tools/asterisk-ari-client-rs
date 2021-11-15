use crate::errors::Result;
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

    /// Get the value of a channel variable
    async fn set_variable(&self, channel_id: &str, var_name: &str, var_value: &str) -> Result<()>;

    /// Hangs up the channel
    async fn hangup(&self, channel_id: &str) -> Result<()>;

    /// Exit application; continue execution in the dialplan
    async fn continue_in_dialplan(&self, channel_id: &str) -> Result<()>;
}
