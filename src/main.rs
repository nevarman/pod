mod config;
mod picture_io;
mod modifiers;

use clap::Parser;
use config::Config;
use modifiers::{metadata_modifier::MetaDataModifier, size_modifier::SizeModifier, ImageModifier};

fn main() {
    // parse command line arguments
    let config = Config::parse();
    println!("{:?}", config);

    let (buffer, metadata) = picture_io::get_picture_of_day_with_metadata(&config).expect("Failed to fetch image");
    println!("Hacking complete");
    let mut image = image::load_from_memory(&buffer).expect("Failed to load image");

    println!("Applying modifiers");
    // apply size_modifier first
    if config.fit_to_screen_size.unwrap_or(false) {
        // get screen size
        // TODO from actual screen?
        let w = config.width.expect("need a target width");
        let h = config.height.expect("need a target height");
        let size_modifier = SizeModifier::new(w, h);
        let result = size_modifier.modify(&mut image);
        if result.is_err() {
            println!("Failed to resize image");
        }
    }
    // apply metadata_modifier next
    if config.add_metadata.unwrap_or(false) {
        let metadata_modifier = MetaDataModifier::new(metadata, &config);
        let result = metadata_modifier.modify(&mut image);
        if result.is_err() {
            println!("Failed to add metadata");
        }
    }
    // save and set background
    let path = config.get_picture_file_name();
    println!("Saving picture to: {}", path);
    image.save_with_format(&path, image::ImageFormat::Jpeg).expect("Failed to save image");
    println!("Setting wallpaper");
    wallpaper::set_from_path(&path).expect("Failed to set wallpaper");
}
