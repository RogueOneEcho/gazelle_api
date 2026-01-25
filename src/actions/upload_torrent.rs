use crate::client::handle_result;
use crate::{GazelleClient, GazelleError, UploadForm, UploadResponse};
use colored::Colorize;
use log::trace;
use std::time::SystemTime;

impl GazelleClient {
    /// Upload a torrent
    ///
    /// # See Also
    ///  - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#upload>
    pub async fn upload_torrent(
        &mut self,
        upload: UploadForm,
    ) -> Result<UploadResponse, GazelleError> {
        let form = upload.to_form().map_err(GazelleError::upload)?;
        self.limiter.execute().await;
        let path = "/ajax.php?action=upload";
        trace!("{} request POST {path}", "Sending".bold());
        let url = format!("{}{path}", self.base_url);
        let start = SystemTime::now();
        let result = self.client.post(&url).multipart(form).send().await;
        let elapsed = start
            .elapsed()
            .expect("elapsed should not fail")
            .as_secs_f64();
        trace!("{} response after {elapsed:.3}", "Received".bold());
        handle_result(result).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{init_logger, load_config};
    use crate::{GazelleClient, GazelleError, UploadForm};
    use std::path::PathBuf;

    #[tokio::test]
    #[ignore = "require upload"]
    async fn upload_torrent_invalid() -> Result<(), GazelleError> {
        // Arrange
        init_logger();
        for (name, config) in load_config() {
            println!("Indexer: {name}");
            let mut client = GazelleClient::from(config.client.clone());
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
                group_id: config.examples.group,
            };

            // Act
            let error = client
                .upload_torrent(form)
                .await
                .expect_err("should be an error");
            println!("{error:?}");

            // Assert
            if name == "ops" {
                assert!(matches!(
                    error,
                    GazelleError::Other {
                        status: 200,
                        message: _
                    }
                ));
            } else {
                assert!(matches!(error, GazelleError::BadRequest { message: _ }));
            }
        }
        Ok(())
    }
}
