use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub provider: Provider,
    pub random: bool,
    pub nasa_api_key: String,
}

#[derive(Debug, PartialEq)]
pub enum Provider {
    Nasa,
    Bing,
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