use crate::{GazelleClient, GazelleError};

impl GazelleClient {
    /// Get the content of the .torrent file as a buffer
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#download>
    pub async fn get_torrent_file_as_buffer(&mut self, id: u32) -> Result<Vec<u8>, GazelleError> {
        let query = format!("action=download&id={id}");
        let result = self.get_internal(query).await;
        let response = result.map_err(GazelleError::Request)?;
        let status_code = response.status();
        if let Some(error) = GazelleError::from_status_code(status_code) {
            return Err(error);
        }
        if status_code.is_success() {
            let bytes = response
                .bytes()
                .await
                .expect("Response should not be empty");
            let buffer = bytes.to_vec();
            Ok(buffer)
        } else {
            let reason = status_code
                .canonical_reason()
                .unwrap_or_default()
                .to_owned();
            Err(GazelleError::Other(status_code, reason))
        }
    }
}
