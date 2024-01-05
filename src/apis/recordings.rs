//! Common module to handle live recording for both channels and bridges.
//! This module doesn't offer method to start recording.
//! Checkout [record](crate::apis::channels::ChannelsAPI#tymethod.record) method in respective channel / bridge.

use crate::errors::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RecordingsAPI {
    /// Stop a live recording and store it.
    async fn stop_recording(&self, recording_name: &str) -> Result<()>;

    /// Pause a live recording.
    async fn pause_recording(&self, recording_name: &str) -> Result<()>;

    /// Unpause a live recording.
    async fn unpause_recording(&self, recording_name: &str) -> Result<()>;

    /// Mute a live recording.
    async fn mute_recording(&self, recording_name: &str) -> Result<()>;

    /// Unmute a live recording.
    async fn unmute_recording(&self, recording_name: &str) -> Result<()>;

    /// Stop a live recording and discard it.
    async fn delete_recording(&self, recording_name: &str) -> Result<()>;
}
