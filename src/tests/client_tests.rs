use crate::tests::load_config;
use crate::GazelleClient;
use rogue_logging::Verbosity::Trace;
use rogue_logging::{Error, Logger, LoggerBuilder};
use std::sync::Arc;

fn init_logger() -> Arc<Logger> {
    LoggerBuilder::new()
        .with_exclude_filter("reqwest".to_owned())
        .with_exclude_filter("cookie".to_owned())
        .with_verbosity(Trace)
        .create()
}

#[tokio::test]
async fn get_torrent() -> Result<(), Error> {
    // Arrange
    init_logger();
    for (name, config) in load_config()? {
        println!("Indexer: {name}");
        let mut client = GazelleClient::from_options(config.client);

        // Act
        let response = client.get_torrent(config.examples.torrent).await?;

        // Assert
        assert_eq!(response.torrent.id, config.examples.torrent);
    }
    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_invalid() -> Result<(), Error> {
    // Arrange
    init_logger();
    let id = u32::MAX;
    for (name, config) in load_config()? {
        println!("Indexer: {name}");
        let mut client = GazelleClient::from_options(config.client.clone());

        // Act
        let response = client.get_torrent(id).await;

        // Assert
        match response {
            Ok(_) => panic!("should be an error"),
            Err(e) => {
                assert_eq!(e.action, "get torrent");
                if name == *"red" {
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
    init_logger();
    for (name, config) in load_config()? {
        println!("Indexer: {name}");
        let mut client = GazelleClient::from_options(config.client.clone());

        // Act
        let response = client.get_torrent_group(config.examples.group).await?;

        // Assert
        assert_eq!(response.group.id, config.examples.group);
    }
    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_group_invalid() -> Result<(), Error> {
    // Arrange
    init_logger();
    let id = u32::MAX;
    for (name, config) in load_config()? {
        println!("Indexer: {name}");
        let mut client = GazelleClient::from_options(config.client.clone());

        // Act
        let response = client.get_torrent_group(id).await;

        // Assert
        match response {
            Ok(_) => panic!("should be an error"),
            Err(e) => {
                assert_eq!(e.action, "get torrent group");
                if name == *"red" {
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
    init_logger();
    for (name, config) in load_config()? {
        println!("Indexer: {name}");
        let mut client = GazelleClient::from_options(config.client.clone());

        // Act
        let user = client.get_user(config.examples.user).await?;

        // Assert
        assert!(!user.username.is_empty());
    }
    Ok(())
}
