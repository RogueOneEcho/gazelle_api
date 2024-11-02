use colored::Colorize;
use log::*;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::time::{Duration, SystemTime};
use tower::limit::RateLimit;
use tower::ServiceExt;

use crate::{GazelleClientFactory, UploadForm, UploadResponse};
use crate::{ApiResponse, GroupResponse, TorrentResponse};

/// A client for the Gazelle API
///
/// Created by an [`GazelleClientFactory`]
pub struct GazelleClient {
    pub api_url: String,
    pub client: RateLimit<Client>,
}

impl GazelleClient {
    /// Get a torrent by id
    ///
    /// A torrent is a specific encoding of a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
    pub async fn get_torrent(&mut self, id: i64) -> Result<TorrentResponse, AppError> {
        let url = format!("{}/ajax.php?action=torrent&id={}", self.api_url, id);
        let response = self.get(&url, "get torrent").await?;
        self.deserialize(response, "get torrent response").await
    }

    /// Get a torrent group by id
    ///
    /// A torrent group is a collection of different encodings of
    /// a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent-group>
    pub async fn get_torrent_group(&mut self, id: i64) -> Result<GroupResponse, AppError> {
        let url = format!("{}/ajax.php?action=torrentgroup&id={}", self.api_url, id);
        let response = self.get(&url, "get torrent group").await?;
        self.deserialize(response, "get torrent group response")
            .await
    }

    /// Get the content of the .torrent file as a buffer
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#download>
    pub async fn get_torrent_file_as_buffer(&mut self, id: i64) -> Result<Vec<u8>, AppError> {
        let url = format!("{}/ajax.php?action=download&id={}", self.api_url, id);
        let response = self.get(&url, "get torrent file").await?;
        let status_code = response.status();
        if status_code.is_success() {
            let bytes = response
                .bytes()
                .await
                .expect("Response should not be empty");
            let buffer = bytes.to_vec();
            Ok(buffer)
        } else {
            AppError::response(status_code, "get torrent file")
        }
    }

    /// Upload the torrent
    ///
    /// # See Also
    ///  - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#upload>
    pub async fn upload_torrent(&mut self, upload: UploadForm) -> Result<UploadResponse, AppError> {
        let url = format!("{}/ajax.php?action=upload", self.api_url);
        let form = upload.to_form()?;
        let client = self.wait_for_client().await;
        let result = client.post(&url).multipart(form).send().await;
        trace!("{} POST request: {}", "Sent".bold(), &url);
        let response = result.or_else(|e| AppError::request(e, "post upload"))?;
        self.deserialize(response, "upload torrent response").await
    }

    async fn get(&mut self, url: &String, action: &str) -> Result<Response, AppError> {
        trace!("{} request GET {}", "Sending".bold(), &url);
        let client = self.wait_for_client().await;
        let start = SystemTime::now();
        let result = client.get(url).send().await;
        let elapsed = start
            .elapsed()
            .expect("elapsed should not fail")
            .as_secs_f64();
        trace!("{} response after {elapsed:.3}", "Received".bold());
        result.or_else(|e| AppError::request(e, action))
    }

    async fn deserialize<T: DeserializeOwned>(
        &mut self,
        response: Response,
        action: &str,
    ) -> Result<T, AppError> {
        let status_code = response.status();
        let json = response.text().await.unwrap_or_default();
        let deserialized = serde_json::from_str::<ApiResponse<T>>(json.as_str())
            .or_else(|e| AppError::json(e, format!("deserialize {action}").as_str()));
        if status_code.is_success() {
            let deserialized = deserialized?;
            if deserialized.status == "success" {
                Ok(deserialized.response.expect("response should be set"))
            } else {
                AppError::explained(action, format!("{deserialized}"))
            }
        } else {
            let message = if let Ok(response) = deserialized {
                response.error.unwrap_or(json)
            } else {
                json
            };
            Err(AppError {
                action: action.to_owned(),
                message,
                status_code: Some(status_code.as_u16()),
                ..AppError::default()
            })
        }
    }

    async fn wait_for_client(&mut self) -> &Client {
        let start = SystemTime::now();
        let client = self
            .client
            .ready()
            .await
            .expect("client should be available")
            .get_ref();
        let duration = start.elapsed().expect("duration should not fail");
        if duration > Duration::from_millis(200) {
            trace!(
                "{} {:.3} for rate limiter",
                "Waited".bold(),
                duration.as_secs_f64()
            );
        }
        client
    }
}
