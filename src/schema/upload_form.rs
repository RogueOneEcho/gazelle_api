use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadForm {
    /// Path to the torrent file to upload
    pub path: PathBuf,
    /// ID number of the category
    ///
    /// *CAUTION: This index is inexplicably different to the [`Group`] `category_id`*
    ///
    /// `Music`: 0,
    /// `Applications`: 1,
    /// `E-Books`: 2,
    /// `Audiobooks`: 3,
    /// `E-Learning Videos`: 4,
    /// `Comedy`: 5,
    /// `Comics`: 6
    /// <https://github.com/OPSnet/Gazelle/blob/3e2f8f8ef99f654047d86ea75da166e270b85ba9/public/static/functions/upload.js#L702-L710>
    pub category_id: u8,
    /// Edition year
    pub remaster_year: u16,
    /// Edition title
    pub remaster_title: String,
    /// Edition record label
    pub remaster_record_label: String,
    /// Edition catalogue number
    pub remaster_catalogue_number: String,
    /// Format
    ///
    /// 0: `MP3`
    /// 1: `FLAC`
    /// 2: `AAC`
    /// 3: `AC3`
    /// 4: `DTS`
    ///
    /// *OPS may have others*
    pub format: String,
    /// Encoding
    ///
    /// 0: `192`
    /// 1: `APS (VBR)`
    /// 2: `V2 (VBR)`
    /// 3: `V1 (VBR)`
    /// 4: `256`
    /// 5: `APX (VBR)`
    /// 6: `V0 (VBR)`
    /// 7: `320`
    /// 8: `Lossless`
    /// 9: `24bit Lossless`
    /// 10: `Other`
    ///
    /// *OPS may have others*
    pub bitrate: String,
    /// Media 
    ///
    /// 0: `CD`
    /// 1: `DVD`
    /// 2: `Vinyl`
    /// 3: `Soundboard`
    /// 4: `SACD`
    /// 5: `DAT`
    /// 6: `Cassette`
    /// 7: `WEB`
    /// 8: `Blu-Ray` (Possibly `Blu-ray` on OPS)
    pub media: String,
    /// Description formatted as BB code
    pub release_desc: String,
    /// ID of the torrentgroup
    pub group_id: u32,
}

impl UploadForm {
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
