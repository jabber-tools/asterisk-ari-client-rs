use crate::errors::Result;
use crate::models::applications::Application;
use async_trait::async_trait;

#[async_trait]
pub trait ApplicationsAPI {
    /// Filter application events types.
    async fn filter(
        &self,
        application_name: &str,
        filter: Option<serde_json::Value>,
    ) -> Result<String>;

    /// Get details of an application.
    async fn get(&self, application_name: &str) -> Result<Application>;

    /// List all applications.
    async fn list(&self) -> Result<Vec<Application>>;

    /// Subscribe an application to a event source.
    async fn subscribe(&self, application_name: &str, event_source: Vec<String>) -> Result<String>;

    /// Unsubscribe an application from an event source.
    async fn unsubscribe(
        &self,
        application_name: &str,
        event_source: Vec<String>,
    ) -> Result<String>;
}
