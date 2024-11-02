use crate::options::get_options;
use crate::Error;
use rogue_logging::Logger;
use crate::Action::{GetTorrent, GetTorrentGroup};

#[tokio::test]
async fn get_torrent() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    for options in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let response = client.get_torrent(options.torrent).await?;

        // Assert
        assert_eq!(response.torrent.id, options.torrent);
    }
    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_invalid() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    let id = u32::MAX;
    for options in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let response = client.get_torrent(id).await;

        // Assert
        match response {
            Ok(_) => panic!("should be an error"),
            Err(e) => {
                assert!(matches!(e.action, GetTorrent));
                println!("{:?}", e.inner);
                assert!(e.inner.is_none());
                if options.name == "red".to_owned() {
                    assert_eq!(e.status_code, Some(400));
                    assert_eq!(e.message, Some("bad id parameter".to_owned()));
                } else {
                    assert_eq!(e.status_code, Some(200));
                    assert_eq!(e.message, Some("bad parameters".to_owned()));
                }
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn get_torrent_group() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    for options in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let response = client.get_torrent_group(options.group).await?;

        // Assert
        assert_eq!(response.group.id, options.group);
    }
    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_group_invalid() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    let id = u32::MAX;
    for options in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let response = client.get_torrent_group(id).await;

        // Assert
        match response {
            Ok(_) => panic!("should be an error"),
            Err(e) => {
                assert!(matches!(e.action, GetTorrentGroup));
                println!("{:?}", e.inner);
                if options.name == "red".to_owned() {
                    assert_eq!(e.status_code, Some(400));
                    assert_eq!(e.message, Some("bad id parameter".to_owned()));
                } else {
                    assert_eq!(e.status_code, Some(200));
                    assert_eq!(e.message, Some("bad parameters".to_owned()));
                }
            }
        }
    }
    Ok(())
}
