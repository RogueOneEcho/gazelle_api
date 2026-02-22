use crate::client::handle_result;
use crate::{GazelleClient, GazelleError, UploadForm, UploadResponse};
use log::trace;
use std::time::SystemTime;

impl GazelleClient {
    /// Upload a torrent
    ///
    /// # See Also
    ///  - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#upload>
    pub async fn upload_torrent(&self, upload: UploadForm) -> Result<UploadResponse, GazelleError> {
        let form = upload.to_form().map_err(GazelleError::upload)?;
        self.limiter.execute().await;
        let path = "/ajax.php?action=upload";
        trace!("Sending request POST {path}");
        let url = format!("{}{path}", self.base_url);
        let start = SystemTime::now();
        let result = self.client.post(&url).multipart(form).send().await;
        let elapsed = start
            .elapsed()
            .expect("elapsed should not fail")
            .as_secs_f64();
        trace!("Received response after {elapsed:.3}");
        handle_result(result).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use crate::tests::for_each_indexer;
    use crate::{ApiResponseKind, GazelleError, GazelleOperation, UploadForm};

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn upload_torrent_invalid() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, examples| async move {
            // Arrange
            let form = UploadForm {
                path: PathBuf::from("/srv/shared/tests/example-1.torrent"),
                category_id: 0,
                remaster_year: 0,
                remaster_title: "ALBUM TITLE".to_owned(),
                remaster_record_label: "RECORD LABEL".to_owned(),
                remaster_catalogue_number: "CATALOGUE NUMBER".to_owned(),
                format: "FLAC".to_owned(),
                bitrate: "Lossless".to_owned(),
                media: "Cassette".to_owned(),
                release_desc: "DESCRIPTION".to_owned(),
                group_id: examples.group,
            };

            // Act
            let error = client
                .lock()
                .await
                .upload_torrent(form)
                .await
                .expect_err("should be an error");
            println!("[{name}] {error:?}");

            // Assert
            let expected = if name == "ops" {
                GazelleOperation::ApiResponse(ApiResponseKind::Other)
            } else {
                GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
            };
            assert_eq!(
                error.operation, expected,
                "[{name}] unexpected error: {error:?}"
            );
            Ok(())
        })
        .await
    }
}
