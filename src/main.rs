use std::{env, process};

use constants::ACCEPTED_MIMETYPES;
use image::{GenericImage, GenericImageView, Pixel};
use indicatif::{ProgressBar, ProgressStyle};
use string_ext::StringUtils;
use verifier::get_mime_index;

mod constants;
mod string_ext;
mod verifier;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        println!("Not enough arguments... Quitting");
        process::exit(1);
    }

    let img_path = &args[0];
    let watermark_path = &args[1];

    let dot_index_1 = match get_mime_index(img_path) {
        Ok(index) => index,
        Err(_) => {
            println!("Invalid path found. Check that your paths are valid...");
            process::exit(1);
        }
    };

    let dot_index_2 = match get_mime_index(watermark_path) {
        Ok(index) => index,
        Err(_) => {
            println!("Invalid path found. Check that your paths are valid...");
            process::exit(1);
        }
    };

    let mime1 = img_path.substring(dot_index_1 + 2, img_path.len());
    let mime2 = watermark_path.substring(dot_index_2 + 2, watermark_path.len());

    if !ACCEPTED_MIMETYPES.contains(&mime1) || !ACCEPTED_MIMETYPES.contains(&mime2) {
        println!("{}", mime1);
        println!("{}", mime2);
        println!("Did not find valid images");
        process::exit(1)
    }

    let mut main_img = image::open(img_path).expect("File not found");
    let watermark = image::open(watermark_path).expect("File not found");

    // let styles = [
    //     ("Rough bar:", "█  ", "red"),
    //     ("Fine bar: ", "█▉▊▋▌▍▎▏  ", "yellow"),
    //     ("Vertical: ", "█▇▆▅▄▃▂▁  ", "green"),
    //     ("Fade in:  ", "█▓▒░  ", "blue"),
    //     ("Blocky:   ", "█▛▌▖  ", "magenta"),
    // ];

    let pb = ProgressBar::new((watermark.width() * watermark.height()) as u64);
    pb.set_style(
        ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", "yellow"))
            .unwrap()
            .progress_chars("█▇▆▅▄▃▂▁  "),
    );
    pb.set_prefix("Appling Watermark");

    let x_padding: u32 = 20;
    let y_padding: u32 = main_img.height() - x_padding;

    let wx_range = (
        main_img.width() - x_padding - watermark.width(),
        main_img.width() - x_padding,
    );

    let wy_range = (
        main_img.height() - y_padding,
        main_img.height() - y_padding + watermark.height(),
    );

    let mut wx: u32 = 0;
    let mut wy: u32 = 0;

    for y in wy_range.0..wy_range.1 {
        for x in wx_range.0..wx_range.1 {
            let mut pixel = main_img.get_pixel(x, y);
            let w_pixel = watermark.get_pixel(wx, wy);

            // Appling Watermark pixel
            // println!("Appling Watermark pixel: {} {}", wx, wy);
            pixel.blend(&w_pixel);
            main_img.put_pixel(x, y, pixel);

            wx += 1;

            if wx == watermark.width() {
                wx = 0;
                wy += 1;

                if wy == watermark.height() {
                    wy = 0;
                }
            }

            pb.inc(1);
            pb.set_message(format!(
                "{:3}%",
                100 * pb.position() / ((watermark.width() * watermark.height()) as u64)
            ));
        }
    }

    match main_img.save_with_format("output.png", image::ImageFormat::Png) {
        Ok(_) => pb.finish_with_message("100% <Operation completed successfully>"),
        Err(_) => pb.finish_with_message("Operation Failed"),
    }
}
