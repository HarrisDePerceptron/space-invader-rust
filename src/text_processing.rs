use ab_glyph::{FontRef, PxScale};
use image::GrayImage;

use crate::container::Container;

pub type Image2d = Vec<Vec<u8>>;

pub fn image_to_vec(img: &image::GrayImage) -> Image2d {
    let mut result: Image2d = vec![];

    for row in 0..img.height() as usize {
        result.push(vec![]);
        for col in 0..img.width() as usize {
            let pix = img.get_pixel(col as u32, row as u32);
            let val = pix.0[0];

            result[row].push(val);
        }
    }

    result
}

pub fn process_text(canvas_container: Container) -> Image2d {
    let font = FontRef::try_from_slice(include_bytes!("../assets/fonts/DejaVuSans.ttf")).unwrap();

    let font_size: f32 = 8.0;

    let scale = PxScale {
        x: font_size,
        y: font_size,
    };

    let scale2 = PxScale {
        x: font_size / 1.1,
        y: font_size / 1.1,
    };

    let width: u32 = 128u32;
    let height: u32 = 128u32;

    let mut img = image::GrayImage::new(width, height);
    //let text = "P L A Y  A G A I N";
    let text = "P L A Y A G A I N";
    let text2 = "P R E S S  A N Y  K E Y  T O C O N T I N U E";

    let (w, h) = imageproc::drawing::text_size(scale, &font, text);

    imageproc::drawing::draw_text_mut(&mut img, image::Luma([255u8]), 0, 0, scale, &font, text);

    imageproc::drawing::draw_text_mut(
        &mut img,
        image::Luma([255u8]),
        0 as i32,
        (h * 3u32) as i32,
        scale2,
        &font,
        text2,
    );

    let (w, h) = imageproc::drawing::text_size(scale, &font, text);

    //for row in 0..img.height() {
    //    for col in 0..img.width() {
    //        let pix = img.get_pixel_mut(col, row);
    //        let val = pix.0[0];
    //    }
    //}

    let target_width = canvas_container.get_width() as u32;
    let target_height = canvas_container.get_height() as u32;
    let resized =
        image::imageops::resize(&img, width, height, image::imageops::FilterType::Triangle);
    resized.save("hello.png").unwrap();

    image_to_vec(&resized)
}
