use crate::config::Config;
use crate::picture_io::PictureProvider;

use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::Value;

pub struct PictureBingProvider;

impl PictureProvider for PictureBingProvider {
    fn get_picturedata_with_metadata(
        &self,
        config: &Config,
    ) -> std::io::Result<(Vec<u8>, super::Metadata)> {
        println!("Hacking Bing server...");
        let _ = config;
        let url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
        // fetch data from NASA API
        let response = reqwest::blocking::get(url).expect("Failed to send request");
        // if response is not succesful print error message and exit
        if !response.status().is_success() {
            eprintln!("Failed to fetch data from bing");
            std::process::exit(1);
        }

        // parse response as json
        let response_text = response.text().unwrap();
        let bing_response =
            from_str::<BingResponse>(&response_text).expect("Failed to parse media type");
        let image = &bing_response.images[0];
        let image_url = format!("https://www.bing.com{}", image.url);
        let image_response = reqwest::blocking::get(&image_url).expect("Failed to send request");
        let bytes = image_response.bytes().expect("Failed to read image bytes");
        let metadata = super::Metadata {
            title: Some(image.title.clone()),
            description: Some(image.copyright.clone()),
        };
        // return image bytes and metadata
        Ok((bytes.to_vec(), metadata))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BingResponse {
    pub images: Vec<Image>,
    pub tooltips: Tooltips,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub startdate: String,
    pub fullstartdate: String,
    pub enddate: String,
    pub url: String,
    pub urlbase: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub title: String,
    pub quiz: String,
    pub wp: bool,
    pub hsh: String,
    pub drk: i64,
    pub top: i64,
    pub bot: i64,
    pub hs: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltips {
    pub loading: String,
    pub previous: String,
    pub next: String,
    pub walle: String,
    pub walls: String,
}
