use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    pub provider: Provider,
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
            self.provider
        )
    }
}

#[derive(Debug, Subcommand)]
pub enum Provider {
    Nasa {
        #[arg(long, default_value = "false")]
        nasa_random: bool,
        
        #[arg(long, short)]
        nasa_api_key: String,
    },
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
