use crate::client::{deserialize, get_result};
use crate::{GazelleClient, GazelleError};
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

impl GazelleClient {
    /// Get the content of the .torrent file as a buffer
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#download>
    pub async fn download_torrent(&mut self, id: u32) -> Result<Vec<u8>, GazelleError> {
        let query = format!("action=download&id={id}");
        let result = self.get_internal(query).await;
        let response = result.map_err(GazelleError::request)?;
        let status_code = response.status();
        let content_type = get_content_type(&response).unwrap_or_default();
        if !content_type.contains("application/x-bittorrent") {
            let json = response.text().await.map_err(GazelleError::response)?;
            let response = deserialize::<Value>(json)?;
            return get_result(status_code, response).map(|_| Vec::new());
        }
        if status_code.is_success() {
            let bytes = response
                .bytes()
                .await
                .expect("Response should not be empty");
            let buffer = bytes.to_vec();
            Ok(buffer)
        } else {
            Err(GazelleError::match_status_error(status_code, None)
                .unwrap_or(GazelleError::other(status_code, None)))
        }
    }
}

fn get_content_type(response: &Response) -> Option<String> {
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)?
        .to_str()
        .ok()?
        .to_owned();
    Some(content_type)
}

#[cfg(test)]
mod tests {
    use crate::tests::{init_logger, load_config};
    use crate::{GazelleClient, GazelleError};

    #[tokio::test]
    async fn download_torrent() -> Result<(), GazelleError> {
        // Arrange
        init_logger();
        for (name, config) in load_config() {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client);

            // Act
            let response = client.download_torrent(config.examples.torrent).await?;

            // Assert
            assert!(!response.is_empty());
        }
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::panic)]
    async fn download_torrent_invalid() -> Result<(), GazelleError> {
        // Arrange
        init_logger();
        let id = u32::MAX;
        for (name, config) in load_config() {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client.clone());

            // Act
            let error = client
                .download_torrent(id)
                .await
                .expect_err("should be an error");
            println!("{error:?}");

            // Assert
            assert!(matches!(error, GazelleError::NotFound { message: _ }));
        }
        Ok(())
    }
}
