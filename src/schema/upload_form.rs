use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use reqwest::multipart::{Form, Part};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadForm {
    pub path: PathBuf,
    pub category_id: u8,
    pub remaster_year: u16,
    pub remaster_title: String,
    pub remaster_record_label: String,
    pub remaster_catalogue_number: String,
    pub format: String,
    pub bitrate: String,
    pub media: String,
    pub release_desc: String,
    pub group_id: u32,
}

impl UploadForm {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_form(self) -> Result<Form, Error> {
        let action = "upload torrent";
        let mut file = File::open(&self.path).map_err(|e| Error {
            action: action.to_owned(),
            message: e.to_string(),
            ..Error::default()
        })?;
        let mut buffer = Vec::new();
        let _size = file.read_to_end(&mut buffer).map_err(|e| Error {
            action: action.to_owned(),
            message: e.to_string(),
            ..Error::default()
        })?;
        let filename = self
            .path
            .file_name()
            .expect("file should have a name")
            .to_string_lossy()
            .to_string();
        let torrent_part = Part::bytes(buffer).file_name(filename);
        let form = Form::new()
            .part("file_input", torrent_part)
            .text("type", self.category_id.to_string())
            .text("remaster", "1") // required by OPS but not RED
            .text("remaster_title", self.remaster_title)
            .text("remaster_record_label", self.remaster_record_label)
            .text("remaster_catalogue_number", self.remaster_catalogue_number)
            .text("remaster_year", self.remaster_year.to_string())
            .text("format", self.format)
            .text("bitrate", self.bitrate)
            .text("media", self.media)
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
