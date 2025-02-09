use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    /// Select a provider
    pub provider: Provider,

    #[arg(long, short)]
    /// Add metadata to the image (title and description)
    pub add_metadata: Option<bool>,
    #[arg(long)]
    /// Metadata rect width, height is unbound
    pub metadata_width: Option<f32>,
    #[arg(long)]
    /// Metadata font: font name from system
    pub metadata_font: Option<String>,
    #[arg(long)]
    /// Metadata font size
    pub metadata_font_size: Option<f32>,


    #[arg(long, short)]
    /// Fit the image to the screen size. This is sometimes useful for nasa images
    pub fit_to_screen_size: Option<bool>,

    #[arg(long)]
    /// Target image width if fit screen size set to true
    pub width: Option<u32>,

    #[arg(long)] 
    /// Target image height if fit screen size set to true
    pub height: Option<u32>,
}

impl Config {
    pub fn get_nasa_url(&self) -> String {
        match &self.provider {
            Provider::Nasa {
                nasa_random,
                nasa_api_key,
            } => {
                if *nasa_random {
                    self.get_nasa_random_url()
                } else {
                    format!(
                        "https://api.nasa.gov/planetary/apod?api_key={}",
                        &nasa_api_key
                    )
                }
            }
            _ => "".to_string(),
        }
    }

    pub fn get_nasa_random_url(&self) -> String {
        format!(
            "https://api.nasa.gov/planetary/apod?api_key={}&count=1",
            match &self.provider {
                Provider::Nasa {
                    nasa_random: _,
                    nasa_api_key,
                } => {
                    nasa_api_key
                }
                _ => panic!("Not supported"),
            }
        )
    }

    pub fn is_nasa_random(&self) -> bool {
        match &self.provider {
            Provider::Nasa {
                nasa_random,
                nasa_api_key: _,
            } => {
                *nasa_random
            }
            _ => false,
        }
    }

    pub fn get_picture_file_name(&self) -> String {
        let current_dir = std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("Error, current directory is not available: {:?}", err);
            // get temp directory
            std::env::temp_dir()
        });
        current_dir.join(format!("{}.jpg", self.provider)).to_str().unwrap().to_string()
    }
}

#[derive(Debug, Subcommand)]
pub enum Provider {
    /// Nasa picture of the day
    Nasa {
        #[arg(long, default_value = "false")]
        /// Set a random picture of the day
        nasa_random: bool,
        
        #[arg(long, short)]
        /// Nasa api key, get at: https://api.nasa.gov/
        nasa_api_key: String,
    },
    /// Bing picture of the day
    Bing,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Provider::Nasa { .. } => write!(f, "nasa"),
            Provider::Bing => write!(f, "bing"),
        }
    }
}
