use crate::picture_io::Metadata;
use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache};
use image::{DynamicImage, GenericImage, GenericImageView};

/// Modifies the dynamic image with the given metadata and returns the modified image
pub fn modify_with_metadata(metadata: Metadata, mut dynamic_image: DynamicImage) -> DynamicImage {
    let mut font_db = cosmic_text::fontdb::Database::new();
    font_db.load_system_fonts();

    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new_with_locale_and_db("En-US".into(), font_db);

    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    const FONT_SIZE: f32 = 22.0;
    const LINE_HEIGHT: f32 = FONT_SIZE * 1.2;
    let metrics = Metrics::new(FONT_SIZE, LINE_HEIGHT);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);

    // Borrow buffer together with the font system for more convenient method calls
    let mut buffer = buffer.borrow_with(&mut font_system);

    // Set a size for the text buffer, in pixels
    let width = 300.0;
    // The height is unbounded
    buffer.set_size(Some(width), None);
    // Attributes indicate what font to choose
    let attrs = Attrs::new().family(Family::Name("Arial".into()));

    // Add some text!
    buffer.set_text(
        format!("{}\n{}", metadata.title, metadata.description).as_str(),
        attrs,
        Shaping::Advanced,
    );

    // Perform shaping as desired
    buffer.shape_until_scroll(true);

    // Default text color (0xFF, 0xFF, 0xFF is white)
    const TEXT_COLOR: Color = Color::rgb(0u8, 255u8, 0u8);

    // Set up the canvas
    let height = LINE_HEIGHT * buffer.layout_runs().count() as f32;
    
    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let color = dynamic_image.get_pixel(x, y);
            println!(   "x: {}, y: {}, color: {:?}", x, y, color);
        }
    }
    // Draw to the canvas
    buffer.draw(&mut swash_cache, TEXT_COLOR, |x: i32, y, w, h, color| {
        let a: u8 = color.a();
        if a == 0 || x < 0 || x >= width as i32 || y < 0 || y >= height as i32 || w != 1 || h != 1 {
            // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
            //println!("x: {}, y: {}, w: {}, h: {}, color: {:?}", x, y, w, h, color);

            return;
        }

        // // Scale by alpha (mimics blending with black)
        let scale = |c: u8| (c as i32 * a as i32 / 255).clamp(0, 255) as u8;

        let r = scale(color.r());
        let g = scale(color.g());
        let b = scale(color.b());
        dynamic_image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
    });

    dynamic_image
}
