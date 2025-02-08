mod config;
mod picture_io;
mod modifiers;

use clap::Parser;
use config::Config;
use modifiers::metadata_modifier;

fn main() {
    // parse command line arguments
    let config = Config::parse();
    println!("{:?}", config);

    let (buffer, metadata) = picture_io::get_picture_of_day_with_metadata(&config).expect("Failed to fetch image");
    let image = image::load_from_memory(&buffer).expect("Failed to load image");
    
    // check config for modifiers
    // todo add modifiers
    let image = metadata_modifier::modify_with_metadata(metadata, image);
    
    image.save("image.jpg").expect("Failed to save image");
    //wallpaper::set_from_path(&image).expect("Failed to set wallpaper");
}
