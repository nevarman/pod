pub mod picture_bing_provider;
pub mod picture_nasa_provider;

use crate::config::{Config, Provider};
use chrono::{DateTime, Utc};
use std::fs;

pub trait PictureProvider {
    fn get_image_path(&self, config: &Config) -> String {
        config.get_picture_file_name()
    }

    fn save_picture(&self, config: &Config) -> std::io::Result<String>;

    fn get_cached_or_save_picture(&self, config: &Config) -> std::io::Result<String> {
        let path = self.get_image_path(config);

        if let Err(_) = fs::metadata(&path) {
            println!("Cache picture not found for today, downloading.");
            return self.save_picture(config);
        }

        println!("Cache picture found for today, checking creation date.");
        match get_creation_time(&path) {
            Ok(creation_time) => {
                if is_created_today(creation_time) {
                    println!("The file was created today.");
                    Ok(path)
                } else {
                    self.save_picture(config)
                }
            }
            Err(e) => {
                eprintln!("Failed to get creation time: {}", e);
                self.save_picture(config)
            }
        }
    }
   
}

fn get_creation_time(path: &str) -> std::io::Result<DateTime<Utc>> {
    let metadata = fs::metadata(path)?;
    let creation_time = metadata.created()?;
    Ok(creation_time.into())
}

fn is_created_today(creation_time: DateTime<Utc>) -> bool {
    let today = Utc::now().date_naive();
    creation_time.date_naive() == today
}

fn get_provider(config: &Config) -> &'static dyn PictureProvider {
    match config.provider {
        // use nasa or bing provider to fetch image
        Provider::Nasa {..} => &picture_nasa_provider::PictureNasaProvider,
        Provider::Bing => &picture_bing_provider::PictureBingProvider,
    }
}

pub fn get_picture_of_day(config: &Config) -> std::io::Result<String> {
    let provider = get_provider(config);
    provider.get_cached_or_save_picture(config)
}
