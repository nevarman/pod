mod config;
mod picture_io;
use config::{Config, Provider};
use wallpaper::set_from_path;

fn main() {
    println!("Hello, world!");

    // read command line arguments and assign to Config struct
    let provider = std::env::args().nth(1).expect("Provider not set").parse::<Provider>().expect("invalid provider");
    let random = std::env::args().nth(2).map_or(false, |arg| arg.parse::<bool>().unwrap_or(false));
    let nasa_api = if provider == Provider::Nasa {
        std::env::args().last().expect("NASA_API_KEY not set")
    } else {
        String::new()
    };
    
    let config =  Config::new(provider, random, nasa_api);
    println!("{:?}", config);

    let image = picture_io::get_picture_of_day(&config).expect("Failed to save image");
    println!("Image saved to: {}", image);
    
    set_from_path(&image).expect("Failed to set wallpaper");
}



