extern crate image;

pub mod locate_stars {
    use image::{DynamicImage, GenericImageView, Rgba};
    use std::cmp::{max, min};
    use std::iter::FromIterator;

    /// Given an image, returns a list of the locations of the brightest stars in the image
    pub fn locate_stars(img: DynamicImage) -> Vec<(u32, u32)> {
        let (width, height) = img.dimensions();

        // Find bright areas
        // I think a closure would be a good way to sort all the star centers (bright 3x3 region of
        // pixels).  In Pythonic pseudocode:
        // sorted_star_centers = sorted(pixels,
        //      key=lambda pixel: sum([get_pixel_brightness(nghbr) for nghbr in get_all_neighbors(pixel)]),
        //      reversed=True)

//        for (x, y, _color) in img.pixels() {
//            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
//                continue;
//            }
//
//            get_neighbor_pixels(x, y, &img, width, height);
//        }

        let mut sorted_star_centers = Vec::from_iter(img.pixels());
        sorted_star_centers.sort_by(|pixel_a, pixel_b| {
            let (x_a, y_a, _color_a) = pixel_a;
            let neighbors_a = get_neighbor_pixels(*x_a, *y_a, &img, width, height);

            let (x_b, y_b, _color_b) = pixel_b;
            let neighbors_b = get_neighbor_pixels(*x_b, *y_b, &img, width, height);

            let mut a_brightness: f64 = 0.0;
            let mut b_brightness: f64 = 0.0;
            for p in neighbors_a.iter() {
                a_brightness += get_pixel_brightness(p);
            }
            for p in neighbors_b.iter() {
                b_brightness += get_pixel_brightness(p);
            }

            a_brightness.partial_cmp(&b_brightness).unwrap().reverse()
        });

        let mut star_locations: Vec<(u32, u32)> = Vec::new();

        for star_center in sorted_star_centers.iter().take(20) {
            let (x, y, color) = star_center;

            star_locations.push((*x, *y));
        }
        star_locations
    }

    fn get_neighbor_pixels(x: u32, y: u32, image: & DynamicImage, image_width: u32, image_height: u32) -> Vec<Rgba<u8>> {
        let mut neighbors: Vec<Rgba<u8>> = Vec::new();
        for col in max(0, (x as i32 - 1i32) as u32)..min((x as i32 + 2i32) as u32, image_width) {
            for row in max(0, (y as i32 - 1i32) as u32)..min((y as i32 + 2i32) as u32, image_height) {
                neighbors.push(image.get_pixel(col, row));
            }
        }
        neighbors
    }

    /// To be used for locating stars
    fn get_pixel_brightness(pixel: & Rgba<u8>) -> f64 {
        (*pixel)[0] as f64 +
        (*pixel)[1] as f64 +
        (*pixel)[2] as f64 // +
//        (*pixel)[3] as f64
    }
}
