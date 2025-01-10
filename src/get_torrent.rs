use crate::{GazelleClient, GazelleError, TorrentResponse};

impl GazelleClient {
    /// Get a torrent by id
    ///
    /// A torrent is a specific encoding of a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
    pub async fn get_torrent(&mut self, id: u32) -> Result<TorrentResponse, GazelleError> {
        self.get(format!("action=torrent&id={id}")).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{init_logger, load_config};
    use crate::{GazelleClient, GazelleError};
    use rogue_logging::Error;

    #[tokio::test]
    async fn get_torrent() -> Result<(), Error> {
        // Arrange
        init_logger();
        for (name, config) in load_config()? {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client);

            // Act
            let response = client
                .get_torrent(config.examples.torrent)
                .await
                .expect("should not error");

            // Assert
            assert_eq!(response.torrent.id, config.examples.torrent);
        }
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::panic)]
    async fn get_torrent_invalid() -> Result<(), Error> {
        // Arrange
        init_logger();
        let id = u32::MAX;
        for (name, config) in load_config()? {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client.clone());

            // Act
            let error = client
                .get_torrent(id)
                .await
                .expect_err("should be an error");
            println!("{error:?}");

            // Assert
            assert!(matches!(error, GazelleError::BadRequest));
        }
        Ok(())
    }
}
