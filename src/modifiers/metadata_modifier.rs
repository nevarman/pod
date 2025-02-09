use crate::picture_io::Metadata;
use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache, Weight};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel};

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

    // Set the text to be displayed
    buffer.set_rich_text(
        [
            (metadata.title.as_str(), attrs.weight(Weight::BOLD)),
            ("\n", attrs),
            (metadata.description.as_str(), attrs),
        ],
        attrs,
        Shaping::Advanced,
    );

    // Perform shaping as desired
    buffer.shape_until_scroll(true);

    // Set up the canvas
    let height = LINE_HEIGHT * buffer.layout_runs().count() as f32;
    let x_offset = dynamic_image.width() as f32 - width;
    let y_offset: f32 = 10.0;

    // calculate average luma of the area where the text will be drawn
    let mut avg_luma: f32 = 0.0;
    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let color: image::Rgba<u8> =
                dynamic_image.get_pixel(x + x_offset as u32, y + y_offset as u32);
            let luma = color.to_luma();
            let luminance = luma[0] as f32 / 255.0;
            avg_luma = (luminance + avg_luma) / 2.0;
        }
    }
    println!("avg_luma: {}", avg_luma);
    let text_color: Color = if avg_luma > 0.5 {
        Color::rgb(0u8, 0u8, 0u8)
    } else {
        Color::rgb(255u8, 255u8, 255u8)
    };
    // Draw to the canvas
    buffer.draw(&mut swash_cache, text_color, |x: i32, y, w, h, color| {
        let a: u8 = color.a();
        if a == 0 || x < 0 || x >= width as i32 || y < 0 || y >= height as i32 || w != 1 || h != 1 {
            // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
            return;
        }

        // // Scale by alpha (mimics blending with black)
        let scale = |c: u8| c; //(c as i32 * a as i32 / 255).clamp(0, 255) as u8;

        let r = scale(color.r());
        let g = scale(color.g());
        let b = scale(color.b());
        // Apply anti-aliasing by blending the text color with the background color
        let bg_color =
            dynamic_image.get_pixel(x as u32 + x_offset as u32, y as u32 + y_offset as u32);
        let blend =
            |fg: u8, bg: u8| ((fg as u32 * a as u32 + bg as u32 * (255 - a as u32)) / 255) as u8;

        let blended_r = blend(r, bg_color[0]);
        let blended_g = blend(g, bg_color[1]);
        let blended_b = blend(b, bg_color[2]);
        let blended_a = a.max(bg_color[3]); // Preserve the maximum alpha value

        dynamic_image.put_pixel(
            x as u32 + x_offset as u32,
            y as u32 + y_offset as u32,
            image::Rgba([blended_r, blended_g, blended_b, blended_a]),
        );
    });

    dynamic_image
}
