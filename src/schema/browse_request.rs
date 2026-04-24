use crate::prelude::*;
use urlencoding::encode;

/// Parameters for the Gazelle browse action.
///
/// - All fields are optional; unset fields are omitted from the query string and the server uses its own defaults
///
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrents-browse>
#[derive(Clone, Debug, Default)]
pub struct BrowseRequest {
    /// Format.
    pub format: Option<Format>,
    /// Encoding.
    ///
    /// Gazelle uses Sphinx fulltext matching for this field, so
    /// [`Quality::Lossless`] will also match [`Quality::Lossless24`].
    pub encoding: Option<Quality>,
    /// Category.
    ///
    /// The API supports multiple pipe-separated categories but only single
    /// category filtering is exposed here.
    pub category: Option<Category>,
    /// Sort field.
    pub order_by: Option<OrderBy>,
    /// Sort direction.
    pub order_way: Option<OrderWay>,
    /// Page number (1-indexed).
    pub page: Option<u32>,
    /// Group results by torrent group.
    pub group_results: Option<bool>,
    /// General search string.
    ///
    /// Matches across artist name, album name, and year.
    pub search: Option<String>,
    /// Artist name.
    pub artist: Option<String>,
    /// Album  or the torrent group name.
    pub album: Option<String>,
    /// Tag.
    pub tags: Option<Vec<String>>,
    /// Tag matching operator.
    pub tags_operator: Option<TagsOperator>,
    /// Original release year.
    pub year: Option<u32>,
    /// Edition or remaster title.
    pub edition_title: Option<String>,
    /// Edition or remaster year.
    pub edition_year: Option<u32>,
    /// Media type.
    pub media: Option<Media>,
    /// Release type.
    pub release_type: Option<ReleaseTypeId>,
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
            parts.push(("encoding", encoding.to_string()));
        }
        if let Some(category) = &self.category {
            parts.push(("filter_cat", category.to_group().to_string()));
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
        if let Some(search) = &self.search {
            parts.push(("searchstr", search.clone()));
        }
        if let Some(artist) = &self.artist {
            parts.push(("artistname", artist.clone()));
        }
        if let Some(album) = &self.album {
            parts.push(("groupname", album.clone()));
        }
        if let Some(tags) = &self.tags {
            parts.push(("taglist", tags.join(",")));
        }
        if let Some(tags_operator) = &self.tags_operator {
            parts.push(("tags_type", tags_operator.as_query().to_owned()));
        }
        if let Some(year) = self.year {
            parts.push(("year", year.to_string()));
        }
        if let Some(edition_title) = &self.edition_title {
            parts.push(("remastertitle", edition_title.clone()));
        }
        if let Some(edition_year) = self.edition_year {
            parts.push(("remasteryear", edition_year.to_string()));
        }
        if let Some(media) = &self.media {
            parts.push(("media", media.to_string()));
        }
        if let Some(release_type) = &self.release_type {
            parts.push(("releasetype", release_type.to_string()));
        }
        parts
            .iter()
            .map(|(k, v)| format!("{k}={}", encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }
}

/// Tag matching operator for browse queries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagsOperator {
    /// Match any.
    Or,
    /// Match all.
    And,
}

impl TagsOperator {
    /// Query parameter value for the Gazelle `tags_type` parameter.
    #[must_use]
    pub fn as_query(&self) -> &'static str {
        match self {
            Self::Or => "0",
            Self::And => "1",
        }
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
            encoding: Some(Quality::Lossless24),
            category: Some(Category::Music),
            order_by: Some(OrderBy::Time),
            order_way: Some(OrderWay::Desc),
            page: Some(1),
            group_results: Some(true),
            ..BrowseRequest::default()
        };

        // Act
        let output = request.to_query();

        // Assert
        assert_eq!(
            output,
            "action=browse&format=FLAC&encoding=24bit%20Lossless&filter_cat=1&order_by=time&order_way=desc&page=1&group_results=1"
        );
    }

    #[test]
    fn browse_request_to_query_search_filters() {
        // Arrange
        let request = BrowseRequest {
            search: Some("test album".to_owned()),
            artist: Some("test artist".to_owned()),
            album: Some("test group".to_owned()),
            tags: Some(vec!["jazz".to_owned(), "piano".to_owned()]),
            tags_operator: Some(TagsOperator::And),
            year: Some(2024),
            edition_title: Some("Deluxe".to_owned()),
            edition_year: Some(2025),
            media: Some(Media::WEB),
            release_type: Some(ReleaseTypeId(1)),
            ..BrowseRequest::default()
        };

        // Act
        let output = request.to_query();

        // Assert
        assert_eq!(
            output,
            "action=browse&searchstr=test%20album&artistname=test%20artist&groupname=test%20group&taglist=jazz%2Cpiano&tags_type=1&year=2024&remastertitle=Deluxe&remasteryear=2025&media=WEB&releasetype=1"
        );
    }

    #[test]
    fn browse_request_to_query_partial_filters() {
        // Arrange
        let request = BrowseRequest {
            artist: Some("Logistics".to_owned()),
            category: Some(Category::Music),
            order_by: Some(OrderBy::Time),
            order_way: Some(OrderWay::Desc),
            page: Some(1),
            group_results: Some(true),
            ..BrowseRequest::default()
        };

        // Act
        let output = request.to_query();

        // Assert
        assert_eq!(
            output,
            "action=browse&filter_cat=1&order_by=time&order_way=desc&page=1&group_results=1&artistname=Logistics"
        );
    }
}
