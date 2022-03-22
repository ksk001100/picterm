use image::{DynamicImage, GenericImageView};

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
