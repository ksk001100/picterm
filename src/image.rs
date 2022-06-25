use crate::utils::ImageMode;
use ansi_rgb::Colorable;
use image::{DynamicImage, GenericImageView, LumaA, Rgba};
use rgb::RGB8;

pub fn image_fit_size(img: &DynamicImage, term_w: u32, term_h: u32) -> (u32, u32) {
    let (img_width, img_height) = img.dimensions();
    let (w, h) = get_dimensions(img_width, img_height, term_w, term_h);
    let h = if h == term_h { h - 1 } else { h };
    (w, h)
}

pub fn get_dimensions(width: u32, height: u32, bound_width: u32, bound_height: u32) -> (u32, u32) {
    let bound_height = 2 * bound_height;

    if width <= bound_width && height <= bound_height {
        return (width, std::cmp::max(1, height / 2 + height % 2));
    }

    let ratio = width * bound_height;
    let nratio = bound_width * height;

    let use_width = nratio <= ratio;
    let intermediate = if use_width {
        height * bound_width / width
    } else {
        width * bound_height / height
    };

    if use_width {
        (bound_width, std::cmp::max(1, intermediate / 2))
    } else {
        (intermediate, std::cmp::max(1, bound_height / 2))
    }
}

pub fn print_term_image(img: DynamicImage, mode: ImageMode) {
    let size = crossterm::terminal::size().unwrap();
    let (w, h) = image_fit_size(&img, size.0 as u32, size.1 as u32);
    let imgbuf = img.resize_exact(w, h, image::imageops::FilterType::Triangle);
    let (width, height) = imgbuf.dimensions();

    match mode {
        ImageMode::Rgba => {
            let imgbuf = imgbuf.to_rgba8();
            for y in 0..height {
                for x in 0..width {
                    let pixel = imgbuf.get_pixel(x, y);
                    let Rgba(data) = *pixel;

                    if data[3] == 0 {
                        print!(" ");
                    } else {
                        let bg = RGB8::new(data[0], data[1], data[2]);
                        print!("{}", " ".bg(bg));
                    }
                }
                println!();
            }
        }
        ImageMode::GrayScale => {
            let imgbuf = imgbuf.to_luma_alpha8();
            for y in 0..height {
                for x in 0..width {
                    let pixel = imgbuf.get_pixel(x, y);
                    let LumaA(data) = *pixel;

                    if data[1] == 0 {
                        print!(" ");
                    } else {
                        print!("{}", " ".bg(RGB8::new(data[0], data[0], data[0])));
                    }
                }
                println!();
            }
        }
    }
}
