pub mod picture_bing_provider;
pub mod picture_nasa_provider;

use crate::config::{Config, Provider};

#[derive(Debug)]
pub struct Metadata {
    pub title: String,
    pub description: String,
}

pub trait PictureProvider {
    /// Fetches picture data and metadata
    fn get_picturedata_with_metadata(&self, config: &Config) -> std::io::Result<(Vec<u8>, Metadata)>;
}

fn get_provider(config: &Config) -> &'static dyn PictureProvider {
    match config.provider {
        // use nasa or bing provider to fetch image
        Provider::Nasa { .. } => &picture_nasa_provider::PictureNasaProvider,
        Provider::Bing => &picture_bing_provider::PictureBingProvider,
    }
}

// /// Fetches picture data and metadata
pub fn get_picture_of_day_with_metadata(config: &Config) -> std::io::Result<(Vec<u8>, Metadata)> {
    let provider = get_provider(config);
    provider.get_picturedata_with_metadata(config)
}
