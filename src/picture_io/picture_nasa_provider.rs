use crate::config::Config;
use crate::picture_io::PictureProvider;

use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use std::fmt::Debug;
use std::io::Error;

pub struct PictureNasaProvider;

impl PictureProvider for PictureNasaProvider {
    fn get_picturedata_with_metadata(
        &self,
        config: &Config,
    ) -> std::io::Result<(Vec<u8>, super::Metadata)> {
        println!("Hacking Nasa server...");
        let response = get_nasa_response(&config);
        // parse response as json
        let data = from_str::<NasaImageResponse>(&response);
        // if data is not parsed successfully print error message and exit
        let data = match data {
            Ok(data) => data,
            Err(_) => {
                // return result with error
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to parse and save image from NASA API",
                ));
            }
        };
        // download image from url
        let image_response = reqwest::blocking::get(&data.hdurl).expect("Failed to send request");
        let bytes = image_response.bytes().expect("Failed to read image bytes");
        let metadata = super::Metadata {
            title: data.title.clone(),
            description: data.explanation.clone(),
        };
        // return image bytes and metadata
        Ok((bytes.to_vec(), metadata))
    }
}

fn get_nasa_response(config: &Config) -> String {
    if config.is_nasa_random() {
        get_nasa_random_response(config)
    } else {
        let url = config.get_nasa_url();
        // fetch data from NASA API
        let response = reqwest::blocking::get(&url).expect("Failed to send request");
        // if response is not succesful print error message and exit
        if !response.status().is_success() {
            eprintln!("Failed to fetch data from NASA API");
            std::process::exit(1);
        }
        // parse response as json
        let response_text = response.text().unwrap();
        println!("{}: {}", url, response_text);
        let media_type =
            from_str::<NasaMediaType>(&response_text).expect("Failed to parse media type");
        if media_type.media_type == "video" {
            get_nasa_random_response(config)
        } else {
            response_text
        }
    }
}

fn get_nasa_random_response(config: &Config) -> String {
    let random_response = |res: String| {
        // trim end of response to remove trailing \n
        let response_text = res.trim_end().to_string();
        // remove beginning and trailing [ and ] from response
        response_text[1..response_text.len() - 1].to_string()
    };
    let url = config.get_nasa_random_url();
    let response = reqwest::blocking::get(&url).expect("Failed to send request");
    // if response is not succesful print error message and exit
    if !response.status().is_success() {
        eprintln!("Failed to fetch data from NASA API");
        std::process::exit(1);
    }
    let response_text = response.text().unwrap();
    random_response(response_text)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NasaImageResponse {
    #[serde(default)]
    pub copyright: String,
    pub date: String,
    pub explanation: String,
    pub hdurl: String,
    #[serde(rename = "media_type")]
    pub media_type: String,
    #[serde(rename = "service_version")]
    pub service_version: String,
    pub title: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NasaVideoResponse {
    pub copyright: String,
    pub date: String,
    pub explanation: String,
    #[serde(rename = "media_type")]
    pub media_type: String,
    #[serde(rename = "service_version")]
    pub service_version: String,
    pub title: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NasaMediaType {
    #[serde(rename = "media_type")]
    pub media_type: String,
}
