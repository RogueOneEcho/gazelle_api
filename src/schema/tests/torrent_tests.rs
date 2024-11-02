use crate::Torrent;
use std::path::PathBuf;

#[test]
fn torrent_get_flacs() {
    // Arrange
    let file_list = r"file1.flac{{{12345}}}|||file2.flac{{{67890}}}|||file with spaces.flac{{{54321}}}|||another_file.flac{{{98765}}}|||/path/to/file.flac{{{11111}}}|||C:\windows\path\file.flac{{{22222}}}|||Disc 1/01. track with period.flac{{{33333}}}|||Disc 1/02. track-with-dash.flac{{{44444}}}|||track_with_underscores.flac{{{55555}}}|||file_with_numbers_123.flac{{{66666}}}|||special&char#file.flac{{{77777}}}|||final_file.flac{{{88888}}}cover.jpg{{{123456}}}|||archive.zip{{{234567}}}|||executable.exe{{{345678}}}|||document.pdf{{{456789}}}|||presentation.pptx{{{567890}}}|||disc-image.iso{{{678901}}}|||compressed.tar.gz{{{789012}}}|||photo.png{{{890123}}}|||audio.mp3{{{901234}}}|||final.zip{{{912345}}}".to_owned();
    let torrent = Torrent {
        file_list,
        ..Torrent::default()
    };

    // Act
    let actual = torrent.get_flacs();

    // Assert
    let expected = vec![
        PathBuf::from("file1.flac"),
        PathBuf::from("file2.flac"),
        PathBuf::from("file with spaces.flac"),
        PathBuf::from("another_file.flac"),
        PathBuf::from("/path/to/file.flac"),
        PathBuf::from(r"C:\windows\path\file.flac"),
        PathBuf::from("Disc 1/01. track with period.flac"),
        PathBuf::from("Disc 1/02. track-with-dash.flac"),
        PathBuf::from("track_with_underscores.flac"),
        PathBuf::from("file_with_numbers_123.flac"),
        PathBuf::from("special&char#file.flac"),
        PathBuf::from("final_file.flac"),
    ];
    assert_eq!(actual, expected);
}
