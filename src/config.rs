use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub provider: Provider,
    random: bool,
    nasa_api_key: String,
}

impl Config {
    pub fn new(provider: Provider, random: bool, nasa_api_key: String) -> Self {
        Config {
            provider,
            random,
            nasa_api_key,
        }
    }

    pub fn get_nasa_url(&self) -> String {
        if self.random {
            self.get_nasa_random_url()
        } else {
            format!(
                "https://api.nasa.gov/planetary/apod?api_key={}",
                self.nasa_api_key
            )
        }
    }

    pub fn get_nasa_random_url(&self) -> String {
        format!(
            "https://api.nasa.gov/planetary/apod?api_key={}&count=1",
            self.nasa_api_key
        )
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

#[derive(Debug, PartialEq)]
pub enum Provider {
    Nasa,
    Bing,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Provider::Nasa => write!(f, "nasa"),
            Provider::Bing => write!(f, "bing"),
        }
    }
}

impl FromStr for Provider {
    type Err = ();

    fn from_str(input: &str) -> Result<Provider, Self::Err> {
        match input.to_lowercase().as_str() {
            "nasa" => Ok(Provider::Nasa),
            "bing" => Ok(Provider::Bing),
            _ => Err(()),
        }
    }
}
