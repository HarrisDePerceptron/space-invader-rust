use ab_glyph::{FontRef, PxScale};

pub fn process_text() {
    let font = FontRef::try_from_slice(include_bytes!("../assets/fonts/DejaVuSans.ttf")).unwrap();

    let font_size: f32 = 24.0;

    let scale = PxScale {
        x: font_size,
        y: font_size,
    };

    let scale2 = PxScale {
        x: font_size / 2.0,
        y: font_size / 2.0,
    };

    let mut img = image::GrayImage::new(128, 128);
    let text = "Game Over";
    let text2 = "Play again?";

    let (w, h) = imageproc::drawing::text_size(scale, &font, text);

    imageproc::drawing::draw_text_mut(&mut img, image::Luma([255u8]), 0, 0, scale, &font, text);

    imageproc::drawing::draw_text_mut(
        &mut img,
        image::Luma([255u8]),
        (w / 2u32) as i32,
        (h * 2u32) as i32,
        scale2,
        &font,
        text2,
    );

    let (w, h) = imageproc::drawing::text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    for row in 0..img.height() {
        for col in 0..img.width() {
            let pix = img.get_pixel_mut(col, row);
            let val = pix.0[0];

            //if val > 100 {
            //    pix.0 = [255];
            //}
        }
    }

    let resized = image::imageops::resize(&img, 512, 512, image::imageops::FilterType::Triangle);
    resized.save("hello.png").unwrap();
}
