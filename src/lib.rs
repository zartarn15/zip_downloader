//! # Overview
//!
//! A simple Rust crate for one-line download and read of ZIP files.
//! providing the content as a String or as bytes.
//!
//! This library features:
//!
//! - Download ZIP files from a specified URL
//! - Validate that the URL points to a ZIP archive
//! - Unpack ZIP files into an allocated buffer
//! - Retrieve file content as a String
//! - Retrieve file content as bytes
//! - Access a specific line of text from the file
//!
//! ## Usage
//!
//! ```
//! use zip_downloader::ZipDownloader;
//!
//! let url = "https://github.com/zartarn15/zip_downloader/raw/refs/heads/master/tests/data/text.zip";
//!
//! // Download and get ZIP file as String
//! let string = ZipDownloader::get(url).unwrap().text().unwrap();
//!
//! // Download and get ZIP file as bytes
//! let bytes = ZipDownloader::get(url).unwrap().bytes();
//!
//! // Download and get 3rd line from ZIP-packed text
//! let line = ZipDownloader::get(url).unwrap().line(3).unwrap();
//!
//! // Download and get 3rd byte from ZIP packed byte file
//! let byte = ZipDownloader::get(url).unwrap().bytes()[3];
//!
//! ```

use crate::Error::*;
use reqwest::blocking;
use std::io::Cursor;
use zip::read::ZipArchive;

/// Main crate structure
#[derive(Clone)]
pub struct ZipDownloader {
    url: String,
    file_data: Vec<u8>,
    unpacked: Vec<u8>,
}

/// Ipmplement ZIP Downloading functions
impl ZipDownloader {
    /// Download and unpack ZIP archive file by URL
    pub fn get(url: &str) -> Result<Self, Error> {
        let mut c = Self::new(url)?;

        c.download()?;
        c.unzip()?;

        Ok(c)
    }

    /// Retrieve file content as a text String
    pub fn text(&self) -> Result<String, Error> {
        let vec = self.unpacked.clone();
        let s = String::from_utf8(vec).map_err(ZipToStr)?;

        Ok(s)
    }

    /// Access a specific line of text from the file
    pub fn line(&self, n: usize) -> Result<String, Error> {
        let text = self.text()?;
        let line = text.lines().nth(n).ok_or(NoSuchLine(n))?;

        Ok(line.to_string())
    }

    /// Unpack ZIP files into an vector
    pub fn bytes(&self) -> Vec<u8> {
        self.unpacked.clone()
    }

    fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            url: url.to_string(),
            file_data: Vec::new(),
            unpacked: Vec::new(),
        })
    }

    fn download(&mut self) -> Result<(), Error> {
        let response = blocking::get(&self.url).map_err(UrlGet)?;
        let headers = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .ok_or(Header)?;
        let content_type = headers.to_str().map_err(HeaderStr)?;

        if !content_type.contains("zip") {
            return Err(NotZipFile(content_type.to_string()));
        }

        if response.status().is_success() {
            let data = response.bytes().map_err(UrlResp)?;
            self.file_data = data.to_vec();
            Ok(())
        } else {
            Err(Download(response.status()))
        }
    }

    fn unzip(&mut self) -> Result<(), Error> {
        let reader = Cursor::new(self.file_data.clone());
        let mut archive = ZipArchive::new(reader).map_err(ZipArc)?;
        let num = archive.len();
        if num > 1 {
            return Err(TooManyFiles(num));
        }

        let mut file = archive.by_index(0).map_err(ZipIdx)?;

        std::io::copy(&mut file, &mut self.unpacked).map_err(IoCopy)?;

        Ok(())
    }
}

/// ZIP downloading and unpacking errors
#[derive(Debug)]
pub enum Error {
    Download(reqwest::StatusCode),
    UrlGet(reqwest::Error),
    UrlResp(reqwest::Error),
    ZipArc(zip::result::ZipError),
    ZipIdx(zip::result::ZipError),
    ZipToStr(std::string::FromUtf8Error),
    IoCopy(std::io::Error),
    Header,
    HeaderStr(reqwest::header::ToStrError),
    NotZipFile(String),
    NoSuchLine(usize),
    TooManyFiles(usize),
}
