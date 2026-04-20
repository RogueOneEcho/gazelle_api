use crate::prelude::*;

impl GazelleClient {
    /// Execute a browse query.
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrents-browse>
    pub async fn browse(&self, request: &BrowseRequest) -> Result<BrowseResponse, GazelleError> {
        self.get(request.to_query()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn browse() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, _examples| async move {
            let request = BrowseRequest {
                format: Some(Format::FLAC),
                category: Some(Category::Music),
                page: Some(1),
                ..BrowseRequest::default()
            };
            let response = client.lock().await.browse(&request).await?;
            assert!(
                !response.results.is_empty(),
                "[{name}] expected at least one result"
            );
            assert_eq!(response.current_page, 1, "[{name}] expected page 1");
            Ok(())
        })
        .await
    }
}
