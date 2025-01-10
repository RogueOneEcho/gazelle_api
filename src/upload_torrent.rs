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
        let form = upload.to_form().map_err(GazelleError::IO)?;
        self.limiter.execute();
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
