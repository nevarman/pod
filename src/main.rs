mod config;
mod image;
use config::{Config, Provider};



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
    let config =  Config {
        provider,
        random : random,
        nasa_api_key: nasa_api       
    };
    println!("{:?}", config);

    let image = image::fetch_image(config).expect("Failed to fetch image");
    println!("Image saved to: {}", image);
    
}



