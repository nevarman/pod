use std::fmt::Debug;
use std::io::Error;

use crate::config::{Config, Provider};
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;

pub fn fetch_image(config: Config) -> Result<String, Error> {
    let image_name = get_image_name();
    // check if any file is found with image_name in current directory
    // if file is found return image_name
    if std::fs::metadata(&image_name).is_ok() {
        return Ok(image_name);
    }
    // else fetch image from provider and save it with image_name
    else {
        match config.provider {
            Provider::Nasa => fetch_nasa_image(config),
            Provider::Bing => fetch_bing_image(config),
        }
    }
    
}

fn get_image_name() -> String {
    // get current date in format dd-mm-yy and return .jpg file name
    let today: String = chrono::Local::now().format("%d-%m-%y").to_string();
    format!("{}.jpg", today)
}

fn fetch_bing_image(config: Config) -> Result<String, Error> {
    let _ = config;
    todo!()
}

fn fetch_nasa_image(config: Config) -> Result<String, Error> {
    let response = get_nasa_response(&config);
    println!("{:?}", response);
    // parse response as json
    let data = from_str::<NasaImageResponse>(&response);
    println!("{:?}", data);
    // if data is not parsed successfully print error message and exit
    let data = match data {
        Ok(data) => data,
        Err(_) => {
            // return result with error
            return Err(Error::new(std::io::ErrorKind::Other, "Failed to parse and save image from NASA API"));
        }
    };

    // download image from url 
    let image_response = reqwest::blocking::get(&data.hdurl).expect("Failed to send request");
    let bytes = image_response.bytes().expect("Failed to read image bytes");
    // save it to file with file name in date format dd-mm-yy.jpg in current directory
    let path = get_image_name();
    std::fs::write(&path, bytes).expect("Failed to save image");
    // return result with success
    Ok(path)

}

fn get_nasa_response(config: &Config) -> String {
    // if config.random use count variable in url to get random image
    let url = if config.random { 
        format!("https://api.nasa.gov/planetary/apod?api_key={}&count=1", config.nasa_api_key)
    } else {
        format!("https://api.nasa.gov/planetary/apod?api_key={}", config.nasa_api_key)
    };

    // fetch data from NASA API
    let response = reqwest::blocking::get(&url).expect("Failed to send request");
    // if response is not succesful print error message and exit
    if !response.status().is_success() {
        eprintln!("Failed to fetch data from NASA API");
        std::process::exit(1);
    }
    
    // parse response as json
    let response_text = response.text().unwrap();
    let media_type = from_str::<NasaMediaType>(&response_text).expect("Failed to parse media type");
    if media_type.media_type == "video" {
        let url = format!("https://api.nasa.gov/planetary/apod?api_key={}&count=1", config.nasa_api_key);
        let response = reqwest::blocking::get(&url).expect("Failed to send request");
        // if response is not succesful print error message and exit
        if !response.status().is_success() {
            eprintln!("Failed to fetch data from NASA API");
            std::process::exit(1);
        }
        let response_text = response.text().unwrap();
        // trim end of response to remove trailing \n
        let response_text = response_text.trim_end().to_string();
        // remove beginning and trailing [ and ] from response
        response_text[1..response_text.len()-1].to_string()
    }
    else {
        response_text
    }
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

// struct Metadata {
//     title: String,
//     description: String,
//     date: String,
//     copyright: String,
// }