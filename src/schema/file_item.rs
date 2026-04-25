use html_escape::decode_html_entities;

/// A single file entry from a Gazelle torrent file list.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileItem {
    /// Relative path of the file within the torrent.
    pub name: String,
    /// Size of the file in bytes.
    pub size: u64,
}

/// Parse a file list string into a vec of [`FileItem`].
///
/// HTML entities in file names are decoded.
#[must_use]
pub fn parse_file_list(file_list: &str) -> Vec<FileItem> {
    file_list
        .split("|||")
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| {
            let (name, rest) = entry.split_once("{{{")?;
            let size_str = rest.strip_suffix("}}}")?;
            let size = size_str.parse::<u64>().ok()?;
            Some(FileItem {
                name: decode_html_entities(name).into_owned(),
                size,
            })
        })
        .collect()
}
