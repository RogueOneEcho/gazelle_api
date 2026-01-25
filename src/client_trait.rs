use async_trait::async_trait;

use crate::{GazelleError, GroupResponse, TorrentResponse, UploadForm, UploadResponse, User};

/// Trait for Gazelle API operations
///
/// Implemented by [`GazelleClient`] for production use and
/// [`MockGazelleClient`] (with `mock` feature) for testing.
#[async_trait]
pub trait GazelleClientTrait: Send + Sync {
    /// Get a torrent by id
    async fn get_torrent(&self, id: u32) -> Result<TorrentResponse, GazelleError>;

    /// Get a torrent group by id
    async fn get_torrent_group(&self, id: u32) -> Result<GroupResponse, GazelleError>;

    /// Get a user by id
    async fn get_user(&self, id: u32) -> Result<User, GazelleError>;

    /// Download torrent file content
    async fn download_torrent(&self, id: u32) -> Result<Vec<u8>, GazelleError>;

    /// Upload a torrent
    async fn upload_torrent(&self, upload: UploadForm) -> Result<UploadResponse, GazelleError>;
}
