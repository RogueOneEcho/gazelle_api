use crate::options::GazelleClientOptions;
use crate::GazelleClient;
use log::info;
use rogue_config::{OptionsProvider, YamlOptionsProvider};
use rogue_logging::{Error, Logger};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ExampleValues {
    pub torrent: u32,
    pub group: u32,
    pub user: u32,
}

fn get_options() -> Result<Vec<(GazelleClientOptions, ExampleValues)>, Error> {
    let clients: Vec<GazelleClientOptions> = YamlOptionsProvider::get()?;
    let examples: Vec<ExampleValues> = YamlOptionsProvider::get()?;
    let vec = clients.into_iter().zip(examples).collect();
    Ok(vec)
}

#[tokio::test]
async fn get_torrent() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    for (options, example) in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = GazelleClient::from_options(options);

        // Act
        let response = client.get_torrent(example.torrent).await?;

        // Assert
        assert_eq!(response.torrent.id, example.torrent);
    }
    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_invalid() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    let id = u32::MAX;
    let options: Vec<GazelleClientOptions> = YamlOptionsProvider::get()?;
    for options in options {
        info!("Indexer: {}", options.name);
        let mut client = GazelleClient::from_options(options.clone());

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
    for (options, example) in get_options()? {
        info!("Indexer: {}", options.name);
        let mut client = GazelleClient::from_options(options);

        // Act
        let response = client.get_torrent_group(example.group).await?;

        // Assert
        assert_eq!(response.group.id, example.group);
    }
    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_group_invalid() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    let id = u32::MAX;
    for (options, _example) in get_options()? {
        info!("Indexer: {}", options.name);
        let mut client = GazelleClient::from_options(options.clone());

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
    for (options, example) in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = GazelleClient::from_options(options);

        // Act
        let user = client.get_user(example.user).await?;

        // Assert
        assert!(!user.username.is_empty());
    }
    Ok(())
}
