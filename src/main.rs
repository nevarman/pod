mod config;
mod picture_io;

use clap::Parser;
use config::Config;

fn main() {
    // parse command line arguments
    let config = Config::parse();
    println!("{:?}", config);

    let image = picture_io::get_picture_of_day(&config).expect("Failed to save image");
    println!("Image saved to: {}", image);
    
    wallpaper::set_from_path(&image).expect("Failed to set wallpaper");
}



