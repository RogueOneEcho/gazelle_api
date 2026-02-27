use crate::client::handle_result;
use crate::{GazelleClient, GazelleError, NewSourceUploadForm, UploadResponse};
use log::trace;
use std::time::SystemTime;

impl GazelleClient {
    /// Upload a new source torrent and create a group.
    ///
    /// # See Also
    ///  - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#upload>
    pub async fn upload_new_source(
        &self,
        upload: NewSourceUploadForm,
    ) -> Result<UploadResponse, GazelleError> {
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
    use crate::{
        ApiResponseKind, GazelleError, GazelleOperation, NewSourceUploadArtist,
        NewSourceUploadEdition, NewSourceUploadForm,
    };

    #[tokio::test]
    #[serial]
    #[ignore = "integration test requiring API credentials"]
    async fn upload_new_source_invalid() -> Result<(), GazelleError> {
        for_each_indexer(|name, client, _examples| async move {
            // Arrange
            let form = NewSourceUploadForm {
                path: PathBuf::from("/srv/shared/tests/example-1.torrent"),
                category_id: 255,
                title: "ALBUM TITLE".to_owned(),
                year: 2024,
                release_type: 255,
                media: "WEB".to_owned(),
                tags: vec!["electronic".to_owned()],
                album_desc: "ALBUM DESCRIPTION".to_owned(),
                release_desc: "RELEASE DESCRIPTION".to_owned(),
                request_id: None,
                image: None,
                edition: NewSourceUploadEdition {
                    unknown_release: true,
                    remaster: None,
                    year: 0,
                    title: String::new(),
                    record_label: String::new(),
                    catalogue_number: String::new(),
                    format: "FLAC".to_owned(),
                    bitrate: "Lossless".to_owned(),
                },
                artists: vec![NewSourceUploadArtist {
                    name: "ARTIST".to_owned(),
                    role: 1,
                }],
            };

            // Act
            let error = client
                .lock()
                .await
                .upload_new_source(form)
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
