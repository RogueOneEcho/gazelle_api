use crate::options::get_options;
use log::info;
use rogue_logging::{Error, Logger};

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
        info!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let response = client.get_torrent(id).await;

        // Assert
        match response {
            Ok(_) => panic!("should be an error"),
            Err(e) => {
                assert_eq!(e.action, "get torrent");
                if options.name == *"red" {
                    assert_eq!(e.status_code, Some(400));
                    assert_eq!(e.message, "bad id parameter".to_owned());
                } else {
                    assert_eq!(e.status_code, Some(200));
                    assert_eq!(e.message, "bad parameters".to_owned());
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
        info!("Indexer: {}", options.name);
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
        info!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let response = client.get_torrent_group(id).await;

        // Assert
        match response {
            Ok(_) => panic!("should be an error"),
            Err(e) => {
                assert_eq!(e.action, "get torrent group");
                if options.name == *"red" {
                    assert_eq!(e.status_code, Some(400));
                    assert_eq!(e.message, "bad id parameter".to_owned());
                } else {
                    assert_eq!(e.status_code, Some(200));
                    assert_eq!(e.message, "bad parameters".to_owned());
                }
            }
        }
    }
    Ok(())
}


#[tokio::test]
async fn get_user() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    for options in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let user = client.get_user(options.user).await?;

        // Assert
        assert!(!user.username.is_empty());
    }
    Ok(())
}
