use crate::{GazelleClient, GazelleError, GroupResponse};

impl GazelleClient {
    /// Get a torrent group by id
    ///
    /// A torrent group is a collection of different encodings of
    /// a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent-group>
    pub async fn get_torrent_group(&mut self, id: u32) -> Result<GroupResponse, GazelleError> {
        self.get(format!("action=torrentgroup&id={id}")).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{init_logger, load_config};
    use crate::{GazelleClient, GazelleError};

    #[tokio::test]
    async fn get_torrent_group() -> Result<(), GazelleError> {
        // Arrange
        init_logger();
        for (name, config) in load_config() {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client);

            // Act
            let response = client.get_torrent_group(config.examples.group).await?;

            // Assert
            assert_eq!(response.group.id, config.examples.group);
        }
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::panic)]
    async fn get_torrent_group_invalid() -> Result<(), GazelleError> {
        // Arrange
        init_logger();
        let id = u32::MAX;
        for (name, config) in load_config() {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client.clone());

            // Act
            let error = client
                .get_torrent_group(id)
                .await
                .expect_err("should be an error");
            println!("{error:?}");

            // Assert
            assert!(matches!(error, GazelleError::BadRequest));
        }
        Ok(())
    }
}
