use colored::Colorize;
use log::*;
use reqwest::{Client, Response};
use rogue_logging::Error;
use serde::de::DeserializeOwned;
use std::time::SystemTime;

use crate::*;

/// A client for the Gazelle API
///
/// Created by an [`GazelleClientFactory`]
pub struct GazelleClient {
    pub base_url: String,
    pub client: Client,
    pub limiter: RateLimiter,
}

impl From<GazelleClientOptions> for GazelleClient {
    /// Create a [`GazelleClient`] from [`GazelleClientOptions`]
    #[must_use]
    fn from(options: GazelleClientOptions) -> GazelleClient {
        let factory = GazelleClientFactory { options };
        factory.create()
    }
}

impl GazelleClient {
    /// Get a torrent by id
    ///
    /// A torrent is a specific encoding of a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
    pub async fn get_torrent(&mut self, id: u32) -> Result<TorrentResponse, Error> {
        let url = format!("{}/ajax.php?action=torrent&id={}", self.base_url, id);
        let action = "get torrent";
        let response = self.get(&url, action).await?;
        handle_response(response, action).await
    }

    /// Get a torrent group by id
    ///
    /// A torrent group is a collection of different encodings of
    /// a release (album, EP, single, etc.).
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent-group>
    pub async fn get_torrent_group(&mut self, id: u32) -> Result<GroupResponse, Error> {
        let url = format!("{}/ajax.php?action=torrentgroup&id={}", self.base_url, id);
        let action = "get torrent group";
        let response = self.get(&url, action).await?;
        handle_response(response, action).await
    }

    /// Get the content of the .torrent file as a buffer
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#download>
    pub async fn get_torrent_file_as_buffer(&mut self, id: u32) -> Result<Vec<u8>, Error> {
        let url = format!("{}/ajax.php?action=download&id={}", self.base_url, id);
        let action = "get torrent file";
        let response = self.get(&url, action).await?;
        let status_code = response.status();
        if status_code.is_success() {
            let bytes = response
                .bytes()
                .await
                .expect("Response should not be empty");
            let buffer = bytes.to_vec();
            Ok(buffer)
        } else {
            Err(Error {
                action: action.to_owned(),
                message: "operation failed".to_owned(),
                status_code: Some(status_code.as_u16()),
                ..Error::default()
            })
        }
    }

    /// Upload the torrent
    ///
    /// # See Also
    ///  - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#upload>
    pub async fn upload_torrent(&mut self, upload: UploadForm) -> Result<UploadResponse, Error> {
        let url = format!("{}/ajax.php?action=upload", self.base_url);
        let form = upload.to_form()?;
        self.limiter.execute();
        trace!("{} POST request: {}", "Sending".bold(), &url);
        let result = self.client.post(&url).multipart(form).send().await;
        let action = "upload torrent";
        let response = result.map_err(|e| Error {
            action: action.to_owned(),
            message: e.to_string(),
            status_code: None,
            ..Error::default()
        })?;
        handle_response(response, action).await
    }

    /// Get a user by id
    ///
    /// # See Also
    /// - <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#user-profile>
    pub async fn get_user(&mut self, id: u32) -> Result<User, Error> {
        let url = format!("{}/ajax.php?action=user&id={}", self.base_url, id);
        let action = "get user";
        let response = self.get(&url, action).await?;
        handle_response(response, action).await
    }

    async fn get(&mut self, url: &String, action: &str) -> Result<Response, Error> {
        self.limiter.execute();
        trace!("{} request GET {}", "Sending".bold(), &url);
        let start = SystemTime::now();
        let result = self.client.get(url).send().await;
        let elapsed = start
            .elapsed()
            .expect("elapsed should not fail")
            .as_secs_f64();
        trace!("{} response after {elapsed:.3}", "Received".bold());
        result.map_err(|e| Error {
            action: action.to_owned(),
            message: e.to_string(),
            status_code: None,
            ..Error::default()
        })
    }
}

async fn handle_response<T: DeserializeOwned>(
    response: Response,
    action: &str,
) -> Result<T, Error> {
    let status_code = response.status();
    let json = response.text().await.map_err(|e| Error {
        action: action.to_owned(),
        message: e.to_string(),
        status_code: None,
        ..Error::default()
    })?;
    // Remove malformed OPS response
    let json = json.replace("\"response\":[],", "");
    if status_code.is_success() {
        let deserialized = deserialize::<T>(&json, action)?;
        if deserialized.status == "success" {
            return Ok(deserialized.response.expect("response should be set"));
        }
    }
    let message = match deserialize::<T>(&json, action) {
        Ok(ApiResponse {
            error: Some(error), ..
        }) => error,
        _ => "Unexpected response.".to_owned(),
    };
    Err(Error {
        action: action.to_owned(),
        message,
        status_code: Some(status_code.as_u16()),
        ..Error::default()
    })
}

fn deserialize<T: DeserializeOwned>(json: &str, action: &str) -> Result<ApiResponse<T>, Error> {
    serde_json::from_str::<ApiResponse<T>>(json).map_err(|e| Error {
        action: action.to_owned(),
        message: e.to_string(),
        status_code: None,
        ..Error::default()
    })
}
