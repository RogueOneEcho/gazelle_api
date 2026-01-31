use crate::{GazelleClient, GazelleError, TorrentResponse};

impl GazelleClient {
    /// Get a torrent by id
    ///
    /// A torrent is a specific encoding of a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
    pub async fn get_torrent(&self, id: u32) -> Result<TorrentResponse, GazelleError> {
        self.get(format!("action=torrent&id={id}")).await
    }
}

#[cfg(test)]
mod tests {
    use crate::GazelleError;
    use crate::tests::for_each_indexer;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn get_torrent() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, examples| async move {
            let response = client.lock().await.get_torrent(examples.torrent).await?;
            assert_eq!(
                response.torrent.id, examples.torrent,
                "[{name}] torrent id mismatch"
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn get_torrent_invalid() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, _examples| async move {
            let error = client
                .lock()
                .await
                .get_torrent(u32::MAX)
                .await
                .expect_err("should be an error");
            assert_eq!(
                error.operation,
                crate::GazelleOperation::ApiResponse(crate::ApiResponseKind::BadRequest),
                "[{name}] expected BadRequest, got {error:?}"
            );
            Ok(())
        })
        .await
    }
}
