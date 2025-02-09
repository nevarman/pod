use crate::modifiers::ImageModifier;
use image::DynamicImage;
pub struct SizeModifier {
    width: u32,
    height: u32,
}

impl SizeModifier {
    pub fn new(width: u32, height: u32) -> Self {
        SizeModifier { width, height }
    }
}

impl ImageModifier for SizeModifier {
    fn modify(&self, mut image: DynamicImage) -> DynamicImage {
        println!("Resizing to {}-{} from {}-{}", self.width, self.height, image.width(), image.height());
        // resize(&image, self.width, self.height, image::imageops::FilterType::Lanczos3);
        // if self.width < image.width() && self.height < image.height() {
        //     println!("given w and h not supported");
        //     return image;
        // }
        image = image.resize_to_fill(self.width, self.height, image::imageops::FilterType::Lanczos3);
        println!("Resized to {}-{}", image.width(), image.height());
        image
    }
}
