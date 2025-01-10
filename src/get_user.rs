use crate::{GazelleClient, GazelleError, User};

impl GazelleClient {
    /// Get a torrent group by id
    ///
    /// A torrent group is a collection of different encodings of
    /// a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent-group>
    pub async fn get_user(&mut self, id: u32) -> Result<User, GazelleError> {
        self.get(format!("action=user&id={id}")).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{init_logger, load_config};
    use crate::{GazelleClient, GazelleError};
    use rogue_logging::Error;

    #[tokio::test]
    async fn get_user() -> Result<(), Error> {
        // Arrange
        init_logger();
        for (name, config) in load_config()? {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client);

            // Act
            let response = client
                .get_user(config.examples.user)
                .await
                .expect("should not error");

            // Assert
            assert!(!response.username.is_empty());
        }
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::panic)]
    async fn get_user_invalid() -> Result<(), Error> {
        // Arrange
        init_logger();
        let id = u32::MAX;
        for (name, config) in load_config()? {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client.clone());

            // Act
            let error = client.get_user(id).await.expect_err("should be an error");
            println!("{error:?}");

            // Assert
            if name == "ops" {
                assert!(matches!(error, GazelleError::BadRequest));
            } else {
                assert!(matches!(error, GazelleError::NotFound));
            }
        }
        Ok(())
    }
}
