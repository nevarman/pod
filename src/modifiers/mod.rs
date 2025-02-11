use image::DynamicImage;
use std::fmt;
use std::error::Error;

pub mod size_modifier;
pub mod metadata_modifier;


pub trait ImageModifier {
    fn modify(&self, image: &mut DynamicImage) -> Result<(), ImageModifierError>;
}

#[derive(Debug)]
pub enum ImageModifierError {
    InvalidMetadata(String), // Example: Invalid metadata provided
    // ConfigError(String),     // Example: Configuration-related errors
    ImageProcessingError(String), // Example: Issues during image manipulation
}

impl fmt::Display for ImageModifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageModifierError::InvalidMetadata(msg) => write!(f, "Invalid metadata: {}", msg),
            // ImageModifierError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ImageModifierError::ImageProcessingError(msg) => write!(f, "Image processing error: {}", msg),
        }
    }
}

impl Error for ImageModifierError {}