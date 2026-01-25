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
    pub async fn download_torrent(&self, id: u32) -> Result<Vec<u8>, GazelleError> {
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
    use crate::tests::for_each_indexer;
    use crate::{GazelleError, GazelleErrorKind};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn download_torrent() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, examples| async move {
            let response = client
                .lock()
                .await
                .download_torrent(examples.torrent)
                .await?;
            assert!(
                !response.is_empty(),
                "[{name}] torrent file should not be empty"
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn download_torrent_invalid() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, _examples| async move {
            let error = client
                .lock()
                .await
                .download_torrent(u32::MAX)
                .await
                .expect_err("should be an error");
            assert_eq!(
                error.kind,
                GazelleErrorKind::NotFound,
                "[{name}] expected NotFound, got {error:?}"
            );
            Ok(())
        })
        .await
    }
}
