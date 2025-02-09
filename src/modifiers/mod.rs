use image::DynamicImage;

pub mod size_modifier;
pub mod metadata_modifier;


pub trait ImageModifier {
    fn modify(&self, image: DynamicImage) -> DynamicImage;
}