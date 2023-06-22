use tokio::fs::File;
use warg_crypto::hash::AnyHash;
use thiserror::Error;
use warg_protocol::registry::PackageId;

pub mod local;

#[derive(Debug, Error)]
pub enum ContentStoreError {
    #[error("content with address `{0}` was not found")]
    ContentNotFound(AnyHash),

    #[error("content store internal error: {0}")]
    ContentStoreInternalError(String),
}

/// Implemented by content stores.
#[axum::async_trait]
pub trait ContentStore: Send + Sync {
    /// Fetch content for a given package.
    async fn fetch_content(
        &self,
        package_id: &PackageId,
        digest: &AnyHash,
    ) -> Result<File, ContentStoreError>;

    /// Store content for a given package.
    async fn store_content(
        &self,
        package_id: &PackageId,
        digest: &AnyHash,
        content: &mut File
    ) -> Result<(), ContentStoreError>;

    async fn content_present(
        &self,
        package_id: &PackageId,
        digest: &AnyHash,
    ) -> Result<bool, ContentStoreError>;
}