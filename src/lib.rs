use crate::Error::*;
use reqwest::blocking;
use std::io::Cursor;
use zip::read::ZipArchive;

#[derive(Clone)]
pub struct ZipDownloader {
    url: String,
    file_data: Vec<u8>,
    unpacked: Vec<u8>,
}

impl ZipDownloader {
    pub fn get(url: &str) -> Result<Self, Error> {
        let mut c = Self::new(url)?;

        c.download()?;
        c.unzip()?;

        Ok(c)
    }

    pub fn as_str(&self) -> Result<String, Error> {
        let vec = self.unpacked.clone();
        let s = String::from_utf8(vec).map_err(ZipToStr)?;

        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
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
        let mut file = archive.by_index(0).map_err(ZipIdx)?;

        std::io::copy(&mut file, &mut self.unpacked).map_err(IoCopy)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Download(reqwest::StatusCode),
    UrlGet(reqwest::Error),
    UrlResp(reqwest::Error),
    ZipArc(zip::result::ZipError),
    ZipIdx(zip::result::ZipError),
    ZipToStr(std::string::FromUtf8Error),
    IoCopy(std::io::Error),
}
