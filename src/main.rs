extern crate image;
mod locate_stars;

use structopt::StructOpt;

use std::path::{Path, PathBuf};
//use locate_stars::locate_stars::locate_stars;
use image::GenericImageView;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open("EkantTakePhotos_small.bmp").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    for pixel in img.pixels() {
        println!("{:?}", pixel);
        break;
    }

    locate_stars::locate_stars::locate_stars(img);

//    // Write the contents of this image to the Writer in PNG format.
//    img.save("test.png").unwrap();
}
