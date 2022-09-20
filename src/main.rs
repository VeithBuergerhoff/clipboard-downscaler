use std::borrow::Cow;

use arboard::{Clipboard, ImageData};
use image::{
    imageops::{resize, FilterType},
    Rgba, RgbaImage,
};

fn main() {
    downscale_image(0.4);
}

fn downscale_image(scaling_factor: f32) {
    let mut clipboard = Clipboard::new().unwrap();
    let clipboard_image = clipboard.get_image();

    match clipboard_image {
        Ok(image) => {
            let width = image.width.try_into().unwrap();
            let height = image.height.try_into().unwrap();

            let rgba = to_rgba(&image.bytes, width, height);


            let new_width = (image.width as f32 * scaling_factor) as u32;
            let new_height = (image.height as f32 * scaling_factor) as u32;
            let downscaled = resize(&rgba, new_width, new_height, FilterType::Lanczos3);

            let new_clipboard_image = ImageData {
                width: new_width as usize,
                height: new_height as usize,
                bytes: Cow::from(downscaled.into_raw()),
            };
            clipboard.set_image(new_clipboard_image).unwrap();
            println!("finished downscaling");
        }
        Err(error) => {
            println!("something went wrong: {}", error);
        }
    }
}

fn to_rgba(raw_image: &Cow<[u8]>, width: u32, height: u32) -> RgbaImage {
    let mut iter = raw_image.iter();
    let mut img = RgbaImage::new(width, height);
    for start in 0..(width * height) {
        let x = start % width;
        let y = start / width;
        img.put_pixel(
            x,
            y,
            Rgba([
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ]),
        );
    }

    img
}
