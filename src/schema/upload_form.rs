use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{Category, Format, Media, Quality};

/// Form data for uploading a torrent to a group
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadForm {
    /// Path to the torrent file to upload
    pub path: PathBuf,
    /// Category
    pub category_id: Category,
    /// Edition year
    pub remaster_year: u16,
    /// Edition title
    pub remaster_title: String,
    /// Edition record label
    pub remaster_record_label: String,
    /// Edition catalogue number
    pub remaster_catalogue_number: String,
    /// Format
    pub format: Format,
    /// Quality
    ///
    /// Referred to as `bitrate` in the upload form.
    pub bitrate: Quality,
    /// Media
    pub media: Media,
    /// Description formatted as BB code
    pub release_desc: String,
    /// ID of the torrentgroup
    pub group_id: u32,
}

impl UploadForm {
    /// Convert to a multipart form for the upload API request
    #[allow(clippy::wrong_self_convention, clippy::absolute_paths)]
    pub fn to_form(self) -> Result<Form, std::io::Error> {
        let mut file = File::open(&self.path)?;
        let mut buffer = Vec::new();
        let _size = file.read_to_end(&mut buffer)?;
        let filename = self
            .path
            .file_name()
            .expect("file should have a name")
            .to_string_lossy()
            .to_string();
        let torrent_part = Part::bytes(buffer).file_name(filename);
        let form = Form::new()
            .part("file_input", torrent_part)
            .text("type", self.category_id.to_upload().to_string())
            .text("remaster", "1") // required by OPS but not RED
            .text("remaster_title", self.remaster_title)
            .text("remaster_record_label", self.remaster_record_label)
            .text("remaster_catalogue_number", self.remaster_catalogue_number)
            .text("remaster_year", self.remaster_year.to_string())
            .text("format", self.format.to_string())
            .text("bitrate", self.bitrate.to_string())
            .text("media", self.media.to_string())
            .text("release_desc", self.release_desc)
            .text("groupid", self.group_id.to_string());
        Ok(form)
    }
}

impl Display for UploadForm {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Ok(yaml) = serde_yaml::to_string(self) {
            yaml
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}
