extern crate libraw;
mod locate_stars;

use structopt::StructOpt;

use std::path::{Path, PathBuf};
use locate_stars::locate_stars::locate_stars;
use std::any::Any;
use libraw::PixelType;
use std::borrow::{Borrow, BorrowMut};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let mut photo = libraw::Image::open(Path::new("EkantTakePhotos.dng")).unwrap();

    photo.unpack().unwrap();

    let raw = photo.raw_pixmap().unwrap();
    let mut pix = raw.pixels();
    let pixel = pix.next().unwrap();
    let pix_type = photo.raw_pixel_type().unwrap();
    let val = pixel.value();

    println!("Value: {}", pixel.value());
    println!("Pixel type: {:?}", pix_type.type_id());
//
//    let color4 = photo.color4_pixmap().unwrap();
//    let mut pix4 = color4.pixels();
//    let pixel4 = pix4.next().unwrap();
//    let pix_type4 = photo.raw_pixel_type().unwrap();
//    let val4 = pixel4.value();
//
//    println!("Value: {:?}", pixel4.value());
//    println!("Pixel type: {:?}", pix_type4.type_id());

//    println!("Getting color3");
//    let color3 = photo.color3_pixmap().unwrap();
//    println!("Getting pix3");
//    let mut pix3 = color3.pixels();
//    println!("Getting pixel3");
//    let mut pixel3 = pix3.next().unwrap();
//    println!("Getting pix_type3");
//    let pix_type3 = photo.raw_pixel_type().unwrap();
////    println!("Getting val3");
////    let val3 = pixel3.value();


//    /////////////////////////////////////////////////////////
//    // This is the provided example
//    photo.unpack().unwrap();
//    let raw = photo.raw_pixmap().unwrap();
//
//    let sum = raw.pixels().fold(0, |accum, pixel| {
//        accum + pixel.value() as usize
//    });
//
//    println!("average pixel brightness = {:.3}", sum as f64 / raw.len() as f64);
//    /////////////////////////////////////////////////////////


    /////////////////////////////////////////////////////////
    // This is my attempt with a color image, rather than raw
    let mut photo = libraw::Image::open(Path::new("EkantTakePhotos.dng")).unwrap();
    photo.unpack().unwrap();

    let raw = photo.color3_pixmap().unwrap();

    println!("Getting pixel data");
    let sum: [u16; 3] = raw.pixels().fold([0, 0, 0], |accum, pixel| {
        [accum[0] + pixel.value()[0], accum[1] + pixel.value()[1], accum[2] + pixel.value()[2]]
    });

    println!("average pixel brightness = {:.3}, {:.3}, {:.3}",
             sum[0] as f64 / raw.len() as f64,
             sum[1] as f64 / raw.len() as f64,
             sum[2] as f64 / raw.len() as f64);
    //////////////////////////////////////////////////////////


    println!("Summing pixel values");

//    let sum: (u16, u16, u16) = color3.pixels().fold((0, 0, 0), |(accum_r, accum_g, accum_b), pixel| {
//        (accum_r + pixel.value()[0], accum_g + pixel.value()[1], accum_b + pixel.value()[2])
//    });

//    let mut sum: (u16, u16, u16) = color3.pixels().fold((0, 0, 0), |(mut accum_r, mut accum_g, mut accum_b), pixel| {
//        let r = pixel.borrow();
//        let g = pixel.borrow().clone();
//        println!("Getting values!");
////        (accum_r + r.value()[0], accum_g + r.value()[1], accum_b + r.value()[2])
//        (accum_r + g.value()[0], accum_g + g.value()[1], accum_b + g.value()[2])
//    });
//    let mut r = 0u16;
//    let mut g = 0u16;
//    let mut b = 0u16;
//    for pixel in pix3 {
//        println!("Adding R values");
//        r += *pixel.borrow().value().get(0).unwrap();
//    }
//
//    println!("Finished summing pixel values");
//
////    println!("average pixel brightness = {:.3}, {:.3}, {:.3}",
////             sum.0 as f64 / color3.len() as f64,
////             sum.1 as f64 / color3.len() as f64,
////             sum.2 as f64 / color3.len() as f64);
//
//
//
//    for pixel in color3.pixels() {
//        for value in pixel.value().iter() {
//            let a  = value.borrow();
//            println!("{:?}", a);
//        }
////        println!("{}", pixel.value().get(0).unwrap());
////        println!("{}", pixel)
//    }
//
//    let mut poop = pixel3.borrow_mut();
//    for val in poop.value().iter() {
//        println!("Val: {}", val);
//    }
//
//    println!("Value: {:?}", poop.value());
//    println!("Pixel type: {:?}", pix_type3.type_id());
}
