pub mod picture_bing_provider;
pub mod picture_nasa_provider;

use crate::config::{Config, Provider};
use chrono::{DateTime, Utc};
use std::fs;

// static file name for image
// static IMAGE_NAME: &str = "original.jpg";

pub trait PictureProvider {
    fn get_image_path(&self, config: &Config) -> String {
        config.get_picture_file_name()
    }

    fn save_picture(&self, config: &Config) -> std::io::Result<String>;

    fn get_cached_or_save_picture(&self, config: &Config) -> std::io::Result<String> {
        let path = self.get_image_path(config);

        let metadata_result = fs::metadata(&path);
        if !metadata_result.is_ok() {
            println!("cache picture not found for today downloading");
            self.save_picture(config)
        } else {
            println!("cache picture found for today, checking create date");
            // Get the file metadata
            match metadata_result {
                Ok(metadata) => {
                    // Get the creation time
                    match metadata.created() {
                        Ok(creation_time) => {
                            // Convert SystemTime to DateTime
                            let creation_time: DateTime<Utc> = creation_time.into();
                            let today = Utc::now().date_naive();

                            // Compare the creation date with today's date
                            if creation_time.date_naive() == today {
                                println!("The file was created today.");
                                return Ok(path);
                            } else {
                                return self.save_picture(config);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to get creation time: {}", e);
                            return self.save_picture(config);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get file metadata: {}", e);
                    return self.save_picture(config);
                }
            }
        }
    }
}

// fn get_provider(config: &Config) -> Result<String, std::io::Error> {
//     match config.provider {
//         // use nasa or bing provider to fetch image
//         Provider::Nasa => picture_nasa_provider::PictureNasaProvider.save_picture(config),
//         Provider::Bing => picture_bing_provider::PictureBingProvider.save_picture(config),
//     }
// }

fn get_provider_(config: &Config) -> &'static dyn PictureProvider {
    match config.provider {
        // use nasa or bing provider to fetch image
        Provider::Nasa => &picture_nasa_provider::PictureNasaProvider,
        Provider::Bing => &picture_bing_provider::PictureBingProvider,
    }
}

pub fn get_picture_of_day(config: &Config) -> std::io::Result<String> {
    let provider = get_provider_(config);
    provider.get_cached_or_save_picture(config)
    // let metadata_result = fs::metadata(IMAGE_NAME);

    // if !metadata_result.is_ok() {
    //     println!("cache picture not found downloading");
    //     get_provider(config)
    // } else {
    //     println!("cache picture found, checking create date");
    //     let metadata = metadata_result.unwrap();
    //     // check metadata is file and created within 24 hours
    //     if metadata.is_file()
    //         && metadata
    //             .created()?
    //             .elapsed()
    //             .unwrap_or_else(|err| {
    //                 println!(
    //                     "Error, create time is not supported or available: {:?}",
    //                     err
    //                 );
    //                 std::time::Duration::from_secs(24 * 60 * 60 + 1)
    //             })
    //             .as_secs()
    //             < 24 * 60 * 60
    //     {
    //         println!("{} was created within the last 24 hours", IMAGE_NAME);
    //         return Ok(get_picture_path()?);
    //     } else {
    //         get_provider(config)
    //     }
    // }
}

// fn get_picture_path() -> std::io::Result<String> {
//     let current_dir = std::env::current_dir()?;
//     let image_path = current_dir.join(&IMAGE_NAME);
//     Ok(image_path.to_string_lossy().into_owned())
// }
