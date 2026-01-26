use crate::{GazelleClient, GazelleError, User};

impl GazelleClient {
    /// Get a user by id.
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#user>
    pub async fn get_user(&self, id: u32) -> Result<User, GazelleError> {
        self.get(format!("action=user&id={id}")).await
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
    async fn get_user() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, examples| async move {
            let response = client.lock().await.get_user(examples.user).await?;
            assert!(
                !response.username.is_empty(),
                "[{name}] username should not be empty"
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn get_user_invalid() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, _examples| async move {
            let error = client
                .lock()
                .await
                .get_user(u32::MAX)
                .await
                .expect_err("should be an error");
            assert!(
                matches!(error, GazelleError::BadRequest { .. }),
                "[{name}] expected BadRequest, got {error:?}"
            );
            Ok(())
        })
        .await
    }
}
