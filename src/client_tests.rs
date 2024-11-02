use crate::options::get_test_client;
use crate::Error;

#[tokio::test]
async fn get_torrent() -> Result<(), Error> {
    // Arrange
    let id = 12345;
    let mut client = get_test_client()?;

    // Act
    let response = client.get_torrent(id).await?;

    // Assert
    assert_eq!(response.torrent.id, id);

    Ok(())
}

#[tokio::test]
#[allow(clippy::panic)]
async fn get_torrent_invalid() -> Result<(), Error> {
    // Arrange
    let id = i64::MAX;
    let mut client = get_test_client()?;

    // Act
    let response = client.get_torrent(id).await;

    // Assert
    match response {
        Ok(_) => panic!("should be an error"),
        Err(e) => {
            assert_eq!(e.status_code, Some(400));
            assert_eq!(e.message, Some("bad id parameter".to_owned()));
        }
    }

    Ok(())
}

#[tokio::test]
async fn get_torrent_group() -> Result<(), Error> {
    // Arrange
    let id = 12345;
    let mut client = get_test_client()?;

    // Act
    let response = client.get_torrent_group(id).await?;

    // Assert
    assert_eq!(response.group.id, id);

    Ok(())
}
