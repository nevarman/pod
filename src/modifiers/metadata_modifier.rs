use crate::picture_io::Metadata;
use crate::{config::Config, modifiers::ImageModifier};

use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel};

pub struct MetaDataModifier<'a> {
    metadata: Metadata,
    config: &'a Config,
}

impl<'a> MetaDataModifier<'a> {
    pub fn new(metadata: Metadata, config: &'a Config) -> Self {
        MetaDataModifier { metadata, config }
    }
}

impl<'a> ImageModifier for MetaDataModifier<'a> {
    fn modify(&self, mut image: DynamicImage) -> DynamicImage {
        let mut font_db = cosmic_text::fontdb::Database::new();
        font_db.load_system_fonts();

        // A FontSystem provides access to detected system fonts, create one per application
        let mut font_system = FontSystem::new_with_locale_and_db("En-US".into(), font_db);

        // A SwashCache stores rasterized glyphs, create one per application
        let mut swash_cache = SwashCache::new();

        let font_size = self.config.metadata_font_size.unwrap_or(22.0);
        let line_height = font_size * 1.2;
        let metrics = Metrics::new(font_size, line_height);

        // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
        let mut buffer = Buffer::new(&mut font_system, metrics);

        // Borrow buffer together with the font system for more convenient method calls
        let mut buffer = buffer.borrow_with(&mut font_system);

        // Set a size for the text buffer, in pixels
        let width = self.config.metadata_width.unwrap_or(300.0);
        // The height is unbounded
        buffer.set_size(Some(width), None);
        // Attributes indicate what font to choose
        let font_name = self.config.metadata_font.as_ref().map_or("Arial", |v| v);
        let attrs = Attrs::new().family(Family::Name(font_name));
        let title = format!("ⓘ {}\n\n",self.metadata.title.as_str());
        
        // Set the text to be displayed
        buffer.set_rich_text(
            [
                (title.as_str(), attrs.metrics(Metrics::new(font_size+2.0, line_height + 2.0 * 1.2))),
                // ("\n", attrs),
                (self.metadata.description.as_str(), attrs),
            ],
            attrs,
            Shaping::Advanced,
        );

        // Perform shaping as desired
        buffer.shape_until_scroll(true);

        // Set up the canvas
        let height = line_height * buffer.layout_runs().count() as f32;
        let x_offset = image.width() as f32 - width;
        let y_offset: f32 = 10.0;

        // calculate average luma of the area where the text will be drawn
        let mut avg_luma: f32 = 0.0;
        for y in 0..height as u32 {
            for x in 0..width as u32 {
                let color: image::Rgba<u8> =
                    image.get_pixel(x + x_offset as u32, y + y_offset as u32);
                let luma = color.to_luma();
                let luminance = luma[0] as f32 / 255.0;
                avg_luma = (luminance + avg_luma) / 2.0;
            }
        }
        
        let text_color: Color = if avg_luma > 0.5 {
            Color::rgb(0u8, 0u8, 0u8)
        } else {
            Color::rgb(255u8, 255u8, 255u8)
        };
        // Draw to the canvas
        buffer.draw(&mut swash_cache, text_color, |x: i32, y, w, h, color| {
            let a: u8 = color.a();
            if a == 0
                || x < 0
                || x >= width as i32
                || y < 0
                || y >= height as i32
                || w != 1
                || h != 1
            {
                // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                return;
            }

            // Apply anti-aliasing by blending the text color with the background color
            let bg_color = image.get_pixel(x as u32 + x_offset as u32, y as u32 + y_offset as u32);
            let blend = |fg: u8, bg: u8| {
                ((fg as u32 * a as u32 + bg as u32 * (255 - a as u32)) / 255) as u8
            };

            let blended_r = blend(color.r(), bg_color[0]);
            let blended_g = blend(color.g(), bg_color[1]);
            let blended_b = blend(color.b(), bg_color[2]);
            let blended_a = a.max(bg_color[3]); // Preserve the maximum alpha value

            image.put_pixel(
                x as u32 + x_offset as u32,
                y as u32 + y_offset as u32,
                image::Rgba([blended_r, blended_g, blended_b, blended_a]),
            );
        });
        println!("Added metadata to image");
        image
    }
}
