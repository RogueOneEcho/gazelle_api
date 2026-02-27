use async_trait::async_trait;

use crate::{
    GazelleClientTrait, GazelleError, GroupResponse, NewSourceUploadForm, TorrentResponse,
    UploadForm, UploadResponse, User,
};

/// Mock client for testing without live API calls
///
/// Set return values using the builder pattern, then use as `dyn GazelleClientTrait`.
#[derive(Clone, Debug)]
#[allow(clippy::struct_field_names)]
pub struct MockGazelleClient {
    get_torrent_returns: Option<Result<TorrentResponse, GazelleError>>,
    get_torrent_group_returns: Option<Result<GroupResponse, GazelleError>>,
    get_user_returns: Option<Result<User, GazelleError>>,
    download_torrent_returns: Option<Result<Vec<u8>, GazelleError>>,
    upload_torrent_returns: Option<Result<UploadResponse, GazelleError>>,
    upload_new_source_returns: Option<Result<UploadResponse, GazelleError>>,
}

impl MockGazelleClient {
    /// Create a new mock client with no configured return values
    #[must_use]
    pub fn new() -> Self {
        Self {
            get_torrent_returns: None,
            get_torrent_group_returns: None,
            get_user_returns: None,
            download_torrent_returns: None,
            upload_torrent_returns: None,
            upload_new_source_returns: None,
        }
    }

    /// Configure the return value for `get_torrent`
    #[must_use]
    pub fn with_get_torrent(mut self, result: Result<TorrentResponse, GazelleError>) -> Self {
        self.get_torrent_returns = Some(result);
        self
    }

    /// Configure the return value for `get_torrent_group`
    #[must_use]
    pub fn with_get_torrent_group(mut self, result: Result<GroupResponse, GazelleError>) -> Self {
        self.get_torrent_group_returns = Some(result);
        self
    }

    /// Configure the return value for `get_user`
    #[must_use]
    pub fn with_get_user(mut self, result: Result<User, GazelleError>) -> Self {
        self.get_user_returns = Some(result);
        self
    }

    /// Configure the return value for `download_torrent`
    #[must_use]
    pub fn with_download_torrent(mut self, result: Result<Vec<u8>, GazelleError>) -> Self {
        self.download_torrent_returns = Some(result);
        self
    }

    /// Configure the return value for `upload_torrent`
    #[must_use]
    pub fn with_upload_torrent(mut self, result: Result<UploadResponse, GazelleError>) -> Self {
        self.upload_torrent_returns = Some(result);
        self
    }

    /// Configure the return value for `upload_new_source`
    #[must_use]
    pub fn with_upload_new_source(mut self, result: Result<UploadResponse, GazelleError>) -> Self {
        self.upload_new_source_returns = Some(result);
        self
    }
}

impl Default for MockGazelleClient {
    /// Create a mock client with all `Ok()` responses configured
    fn default() -> Self {
        Self {
            get_torrent_returns: Some(Ok(TorrentResponse::mock())),
            get_torrent_group_returns: Some(Ok(GroupResponse::mock())),
            get_user_returns: Some(Ok(User::mock())),
            download_torrent_returns: Some(Ok(vec![0xd8, 0x3a, 0x00])),
            upload_torrent_returns: Some(Ok(UploadResponse::mock())),
            upload_new_source_returns: Some(Ok(UploadResponse::mock())),
        }
    }
}

#[async_trait]
impl GazelleClientTrait for MockGazelleClient {
    async fn get_torrent(&self, _id: u32) -> Result<TorrentResponse, GazelleError> {
        self.get_torrent_returns
            .clone()
            .expect("MockGazelleClient: get_torrent_returns not set")
    }

    async fn get_torrent_group(&self, _id: u32) -> Result<GroupResponse, GazelleError> {
        self.get_torrent_group_returns
            .clone()
            .expect("MockGazelleClient: get_torrent_group_returns not set")
    }

    async fn get_user(&self, _id: u32) -> Result<User, GazelleError> {
        self.get_user_returns
            .clone()
            .expect("MockGazelleClient: get_user_returns not set")
    }

    async fn download_torrent(&self, _id: u32) -> Result<Vec<u8>, GazelleError> {
        self.download_torrent_returns
            .clone()
            .expect("MockGazelleClient: download_torrent_returns not set")
    }

    async fn upload_torrent(&self, _upload: UploadForm) -> Result<UploadResponse, GazelleError> {
        self.upload_torrent_returns
            .clone()
            .expect("MockGazelleClient: upload_torrent_returns not set")
    }

    async fn upload_new_source(
        &self,
        _upload: NewSourceUploadForm,
    ) -> Result<UploadResponse, GazelleError> {
        self.upload_new_source_returns
            .clone()
            .expect("MockGazelleClient: upload_new_source_returns not set")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;

    use tokio::sync::Mutex;

    use super::*;

    #[tokio::test]
    async fn mock_get_torrent_returns_configured_value() {
        // Arrange
        let expected = TorrentResponse::mock();
        let mock = MockGazelleClient::new().with_get_torrent(Ok(expected.clone()));

        // Act
        let result = mock.get_torrent(123).await;

        // Assert
        assert!(result.is_ok());
        let response = result.expect("should be ok");
        assert_eq!(response.torrent.id, expected.torrent.id);
    }

    #[tokio::test]
    async fn mock_get_torrent_returns_error() {
        // Arrange
        let mock = MockGazelleClient::new()
            .with_get_torrent(Err(GazelleError::not_found("not found".to_owned(), 404)));

        // Act
        let result = mock.get_torrent(999).await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.operation,
            crate::GazelleOperation::ApiResponse(crate::ApiResponseKind::NotFound)
        );
    }

    #[tokio::test]
    async fn mock_get_user_returns_configured_value() {
        // Arrange
        let expected = User::mock();
        let mock = MockGazelleClient::new().with_get_user(Ok(expected.clone()));

        // Act
        let result = mock.get_user(1).await;

        // Assert
        assert!(result.is_ok());
        let user = result.expect("should be ok");
        assert_eq!(user.username, expected.username);
    }

    #[tokio::test]
    async fn mock_download_torrent_returns_bytes() {
        // Arrange
        let expected_bytes = vec![0xd8, 0x3a, 0x00]; // Some bytes
        let mock = MockGazelleClient::new().with_download_torrent(Ok(expected_bytes.clone()));

        // Act
        let result = mock.download_torrent(123).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), expected_bytes);
    }

    #[tokio::test]
    async fn mock_works_as_trait_object() {
        // Arrange - Create mock as trait object for dependency injection
        let response = TorrentResponse::mock();
        let mock = MockGazelleClient::new().with_get_torrent(Ok(response));

        // Use as dyn GazelleClientTrait for dependency injection pattern
        let client: Arc<Mutex<dyn GazelleClientTrait>> = Arc::new(Mutex::new(mock));

        // Act - Use the trait object
        let result = client.lock().await.get_torrent(123).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn mock_can_be_called_multiple_times() {
        // Arrange
        let expected = TorrentResponse::mock();
        let mock = MockGazelleClient::new().with_get_torrent(Ok(expected.clone()));

        // Act - Call multiple times
        let result1 = mock.get_torrent(123).await;
        let result2 = mock.get_torrent(456).await;
        let result3 = mock.get_torrent(789).await;

        // Assert - All calls return the same configured value
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        assert_eq!(
            result1.expect("should be ok").torrent.id,
            expected.torrent.id
        );
        assert_eq!(
            result2.expect("should be ok").torrent.id,
            expected.torrent.id
        );
        assert_eq!(
            result3.expect("should be ok").torrent.id,
            expected.torrent.id
        );
    }

    #[tokio::test]
    async fn mock_default_has_all_ok_responses() {
        // Arrange
        let mock = MockGazelleClient::default();

        // Act & Assert - All methods return Ok
        assert!(mock.get_torrent(1).await.is_ok());
        assert!(mock.get_torrent_group(1).await.is_ok());
        assert!(mock.get_user(1).await.is_ok());
        assert!(mock.download_torrent(1).await.is_ok());
        assert!(
            mock.upload_torrent(crate::UploadForm {
                path: PathBuf::new(),
                category_id: 0,
                remaster_year: 2020,
                remaster_title: String::new(),
                remaster_record_label: String::new(),
                remaster_catalogue_number: String::new(),
                format: String::new(),
                bitrate: String::new(),
                media: String::new(),
                release_desc: String::new(),
                group_id: 1,
            })
            .await
            .is_ok()
        );
        assert!(
            mock.upload_new_source(crate::NewSourceUploadForm {
                path: PathBuf::new(),
                category_id: 0,
                title: "Example".to_owned(),
                year: 2024,
                release_type: 1,
                media: "WEB".to_owned(),
                tags: vec!["electronic".to_owned()],
                album_desc: String::new(),
                release_desc: String::new(),
                request_id: None,
                image: None,
                edition: crate::NewSourceUploadEdition {
                    unknown_release: true,
                    remaster: None,
                    year: 0,
                    title: String::new(),
                    record_label: String::new(),
                    catalogue_number: String::new(),
                    format: "FLAC".to_owned(),
                    bitrate: "Lossless".to_owned(),
                },
                artists: vec![crate::NewSourceUploadArtist {
                    name: "Artist".to_owned(),
                    role: 1,
                }],
            })
            .await
            .is_ok()
        );
    }
}
