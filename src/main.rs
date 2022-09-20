use std::borrow::Cow;

use arboard::{Clipboard, ImageData};
use image::{
    imageops::{resize, FilterType},
    Rgba, RgbaImage,
};
use {std::sync::mpsc, tray_item::TrayItem};

enum Message {
    Quit,
}

fn main() {
    let mut tray = TrayItem::new("Clipboard Downscaler", "main-icon").unwrap();

    tray.add_menu_item("Downscale", || {
        downscale_image();
    })
    .unwrap();

    let (tx, rx) = mpsc::channel();

    tray.add_menu_item("Quit", move || {
        tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => break,
            _ => {}
        }
    }
}

fn downscale_image() {
    let mut clipboard = Clipboard::new().unwrap();
    let clipboard_image = clipboard.get_image();

    match clipboard_image {
        Ok(image) => {
            let width = image.width.try_into().unwrap();
            let height = image.height.try_into().unwrap();
            let mut rgba = to_rgba(&image.bytes, width, height);
            let downscaled = resize(&mut rgba, width / 4, height / 4, FilterType::Lanczos3);
            clipboard
                .set_image(to_clipboard_image(downscaled, width / 4, height / 4))
                .unwrap();
        }
        Err(error) => {
            println!("something went wrong: {}", error);
        }
    }
}

fn to_clipboard_image<'a>(rgba: RgbaImage, width: u32, height: u32) -> ImageData<'a> {
    ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::from(rgba.into_raw()),
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
