use crate::modifiers::{ImageModifier, ImageModifierError};
use image::DynamicImage;

/// A modifier that resizes an image to the specified dimensions.
///
/// # Fields:
/// - `target_width`: The desired width of the image in pixels.
/// - `target_height`: The desired height of the image in pixels.
///
/// This modifier validates the target dimensions and resizes the image using
/// a high-quality Lanczos3 filter.
pub struct SizeModifier {
    width: u32,
    height: u32,
}

impl SizeModifier {
    pub fn new(width: u32, height: u32) -> Self {
        SizeModifier { width, height }
    }

    fn validate_dimensions(&self) -> Result<(), ImageModifierError> {
        if self.width == 0 || self.height == 0 {
            Err(ImageModifierError::ImageProcessingError(
                "Width and height must be greater than zero".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

impl ImageModifier for SizeModifier {
    fn modify(&self, image: &mut DynamicImage) -> Result<(), ImageModifierError> {
        // Validate dimensions
        self.validate_dimensions()?;
        println!(
            "Resizing to {}-{} from {}-{}",
            self.width,
            self.height,
            image.width(),
            image.height()
        );
        
        *image = image.resize_to_fill(
            self.width,
            self.height,
            image::imageops::FilterType::Lanczos3,
        );
        println!("Resized to {}-{}", image.width(), image.height());
        Ok(())
    }
}
