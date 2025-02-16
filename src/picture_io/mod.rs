pub mod picture_bing_provider;
pub mod picture_nasa_provider;

use crate::config::{Config, Provider};

#[derive(Debug)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub trait PictureProvider {
    /// Fetches picture data and metadata
    fn get_picture_with_metadata(&self, config: &Config) -> std::io::Result<(Vec<u8>, Metadata)>;
}

fn get_provider(config: &Config) -> Box<dyn PictureProvider> {
    match config.provider {
        Provider::Nasa { .. } => Box::new(picture_nasa_provider::PictureNasaProvider::new()),
        Provider::Bing => Box::new(picture_bing_provider::PictureBingProvider::new()),
    }
}

/// Fetches picture data and metadata
pub fn get_picture_of_day_with_metadata(config: &Config) -> std::io::Result<(Vec<u8>, Metadata)> {
    let provider = get_provider(config);
    provider.get_picture_with_metadata(config)
}
