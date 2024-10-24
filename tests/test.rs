use zip_downloader::{Error, ZipDownloader};

const TEXT_ZIP: &str =
    "https://github.com/zartarn15/zip_downloader/raw/refs/heads/master/tests/data/text.zip";
const TEXT_TXT: &str =
    "https://github.com/zartarn15/zip_downloader/raw/refs/heads/master/tests/data/text.txt";
const BYTES_ZIP: &str =
    "https://github.com/zartarn15/zip_downloader/raw/refs/heads/master/tests/data/bytes.zip";
const BYTES_BIN: &str =
    "https://github.com/zartarn15/zip_downloader/raw/refs/heads/master/tests/data/bytes.bin";
const TWO_TEXT_ZIP: &str =
    "https://github.com/zartarn15/zip_downloader/raw/refs/heads/master/tests/data/2files_in_dir.zip";

#[test]
fn text_zip_download_test() {
    let data = ZipDownloader::get(TEXT_ZIP).unwrap().text().unwrap();
    let content = reqwest::blocking::get(TEXT_TXT).unwrap().text().unwrap();
    assert_eq!(data, content);
}

#[test]
fn bytes_zip_download_test() {
    let data = ZipDownloader::get(BYTES_ZIP).unwrap().bytes();
    let content = reqwest::blocking::get(BYTES_BIN).unwrap().bytes().unwrap();
    assert_eq!(data, content);
}

#[test]
fn line_zip_download_test() {
    let data = ZipDownloader::get(TEXT_ZIP).unwrap().line(3).unwrap();
    let content = reqwest::blocking::get(TEXT_TXT).unwrap().text().unwrap();
    assert_eq!(data, content.lines().nth(3).unwrap());
}

#[test]
fn no_such_text_line_test() {
    let ret = ZipDownloader::get(TEXT_ZIP).unwrap().line(225);
    assert!(matches!(ret, Err(Error::NoSuchLine(225))));
}

#[test]
fn get_bytes_as_text_test() {
    let ret = ZipDownloader::get(BYTES_ZIP).unwrap().text();
    assert!(matches!(ret, Err(Error::ZipToStr(_))));
}

#[test]
fn http_page_url_test() {
    let ret = ZipDownloader::get("https://github.com/");
    assert!(matches!(ret, Err(Error::NotZipFile(_))));
}

#[test]
fn text_file_url_test() {
    let ret = ZipDownloader::get(TEXT_TXT);
    assert!(matches!(ret, Err(Error::NotZipFile(_))));
}

#[test]
fn wrong_file_url_test() {
    let ret = ZipDownloader::get("https://github.abcdef/file.zip");
    assert!(matches!(ret, Err(Error::UrlGet(_))));
}

#[test]
fn two_files_in_dir_test() {
    let ret = ZipDownloader::get(TWO_TEXT_ZIP);
    assert!(matches!(ret, Err(Error::TooManyFiles(3))));
}
