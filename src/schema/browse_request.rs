use crate::{Category, Format, OrderBy, OrderWay};

/// Parameters for the Gazelle browse action.
///
/// - All fields are optional; unset fields are omitted from the query string and the server uses its own defaults
///
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrents-browse>
#[derive(Clone, Debug, Default)]
pub struct BrowseRequest {
    /// Format filter; serialized as `format=<Format display>`.
    pub format: Option<Format>,
    /// Encoding filter; serialized as `encoding=<value>`.
    ///
    /// - Gazelle's `search_basic` recognizes magic keywords here, notably `24bit`
    /// - See `app/Search/Torrent.php:519` in the Gazelle source
    /// - The response still uses the canonical `Quality` values (e.g. `24bit Lossless`)
    pub encoding: Option<String>,
    /// Category filter; serialized pipe-separated as `filter_cat=1|2|...`.
    pub filter_cat: Option<Vec<Category>>,
    /// Sort field.
    pub order_by: Option<OrderBy>,
    /// Sort direction.
    pub order_way: Option<OrderWay>,
    /// Page number (1-indexed).
    pub page: Option<u32>,
    /// Group results by torrent group.
    pub group_results: Option<bool>,
}

impl BrowseRequest {
    /// Encode the request as a query string suitable for `GazelleClient::get`.
    ///
    /// - The leading `action=browse` is included
    #[must_use]
    pub fn to_query(&self) -> String {
        let mut parts: Vec<(&str, String)> = vec![("action", "browse".to_owned())];
        if let Some(format) = &self.format {
            parts.push(("format", format.to_string()));
        }
        if let Some(encoding) = &self.encoding {
            parts.push(("encoding", encoding.clone()));
        }
        if let Some(cats) = &self.filter_cat {
            let joined = cats
                .iter()
                .map(|c| c.to_group().to_string())
                .collect::<Vec<_>>()
                .join("|");
            parts.push(("filter_cat", joined));
        }
        if let Some(order_by) = &self.order_by {
            parts.push(("order_by", order_by.as_query().to_owned()));
        }
        if let Some(order_way) = &self.order_way {
            parts.push(("order_way", order_way.as_query().to_owned()));
        }
        if let Some(page) = self.page {
            parts.push(("page", page.to_string()));
        }
        if let Some(group_results) = self.group_results {
            parts.push((
                "group_results",
                if group_results { "1" } else { "0" }.to_owned(),
            ));
        }
        parts
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browse_request_to_query_empty() {
        let request = BrowseRequest::default();
        let output = request.to_query();
        assert_eq!(output, "action=browse");
    }

    #[test]
    fn browse_request_to_query_full() {
        // Arrange
        let request = BrowseRequest {
            format: Some(Format::FLAC),
            encoding: Some("24bit".to_owned()),
            filter_cat: Some(vec![Category::Music]),
            order_by: Some(OrderBy::Time),
            order_way: Some(OrderWay::Desc),
            page: Some(1),
            group_results: Some(true),
        };

        // Act
        let output = request.to_query();

        // Assert
        assert_eq!(
            output,
            "action=browse&format=FLAC&encoding=24bit&filter_cat=1&order_by=time&order_way=desc&page=1&group_results=1"
        );
    }

    #[test]
    fn browse_request_to_query_multi_cat() {
        // Arrange
        let request = BrowseRequest {
            filter_cat: Some(vec![
                Category::Music,
                Category::Applications,
                Category::Audiobooks,
            ]),
            ..Default::default()
        };

        // Act
        let output = request.to_query();

        // Assert
        assert_eq!(output, "action=browse&filter_cat=1|2|4");
    }
}
