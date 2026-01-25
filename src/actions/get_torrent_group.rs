use crate::{GazelleClient, GazelleError, GroupResponse};

impl GazelleClient {
    /// Get a torrent group by id
    ///
    /// A torrent group is a collection of different encodings of
    /// a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent-group>
    pub async fn get_torrent_group(&self, id: u32) -> Result<GroupResponse, GazelleError> {
        self.get(format!("action=torrentgroup&id={id}")).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::for_each_indexer;
    use crate::{GazelleError, GazelleErrorKind};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn get_torrent_group() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, examples| async move {
            let response = client
                .lock()
                .await
                .get_torrent_group(examples.group)
                .await?;
            assert_eq!(
                response.group.id, examples.group,
                "[{name}] group id mismatch"
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn get_torrent_group_invalid() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, _examples| async move {
            let error = client
                .lock()
                .await
                .get_torrent_group(u32::MAX)
                .await
                .expect_err("should be an error");
            assert_eq!(
                error.kind,
                GazelleErrorKind::BadRequest,
                "[{name}] expected BadRequest, got {error:?}"
            );
            Ok(())
        })
        .await
    }
}
