use std::{env, fs, process, thread};

use constants::ACCEPTED_MIMETYPES;
use helpers::get_dirs_images_paths;
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use structures::LoadedImage;

mod constants;
mod helpers;
mod structures;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Not enough arguments... Quitting");
        process::exit(1);
    }

    let (watermark_path, outdir): (String, String) =
        if !args.contains(&"-o".to_string()) && !args.contains(&"--out".to_string()) {
            (
                args.pop()
                    .expect("Popped last arg, this message should not ever log"),
                "".to_string(),
            )
        } else {
            let out = args.pop().unwrap();
            args.pop();
            (
                args.pop()
                    .expect("Popped last arg, this message should not ever log"),
                out.trim().to_string(),
            )
        };

    let image_paths: Vec<String> =
        if &args[0] == "-d" || &args[0] == "-dir" || &args[0] == "--directory" {
            let directories_paths: Vec<&String> = args.iter().skip(1).collect();
            get_dirs_images_paths(directories_paths)
        } else {
            args.clone()
        };

    for img_path in &image_paths {
        let index = match img_path.rfind('.') {
            Some(index) => index,
            None => {
                println!("Invalid path found. Check that your paths are valid...");
                process::exit(1);
            }
        };

        let mime = &img_path[index + 1..];

        if !ACCEPTED_MIMETYPES
            .map(|m| m.extension)
            .as_slice()
            .contains(&mime)
        {
            println!("Did not find valid images for {}", mime);
            process::exit(1)
        }
    }

    let images_to_process: Vec<LoadedImage> = image_paths
        .iter()
        .map(|path| LoadedImage {
            data: image::open(path).expect("Could not load image"),
            filename: path[path.rfind('/').expect("Invalid Path") + 1
                ..path.rfind('.').expect("Invalid Path Ext")]
                .to_string(),
            ext: path[path.rfind('.').expect("Invalid Ext") + 1..].to_string(),
        })
        .collect();

    let watermark: DynamicImage =
        image::open(watermark_path).expect("Could not load watermark file");

    let styles = [
        ("Rough bar:", "█  ", "red"),
        ("Fine bar: ", "█▉▊▋▌▍▎▏  ", "yellow"),
        ("Vertical: ", "█▇▆▅▄▃▂▁  ", "green"),
        ("Fade in:  ", "█▓▒░  ", "blue"),
        ("Blocky:   ", "█▛▌▖  ", "magenta"),
    ];

    let multi_bars = MultiProgress::new();

    let handles: Vec<_> = images_to_process
        .iter()
        .map(|image_to_process| {
            let pb = multi_bars.add(ProgressBar::new(
                (image_to_process.data.width() * image_to_process.data.height()) as u64,
            ));
            let syl = styles
                .choose(&mut rand::thread_rng())
                .expect("Could not choose style");
            pb.set_style(
                ProgressStyle::with_template(&format!(
                    "{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}",
                    syl.2
                ))
                .unwrap()
                .progress_chars(syl.1),
            );
            pb.set_prefix(format!(
                "Appling Watermark for {}.{}",
                image_to_process.filename, image_to_process.ext,
            ));

            let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(
                image_to_process.data.width(),
                image_to_process.data.height(),
            );

            let processing_image = image_to_process.data.clone();
            let processing_watermark = watermark.clone();
            let filename = image_to_process.filename.clone();
            let extension = image_to_process.ext.clone();
            let path = if !outdir.is_empty() {
                match fs::create_dir_all(outdir.clone()) {
                    Ok(_) => format!("{}/{}_WATERMARKED.{}", outdir, filename, extension),
                    Err(_) => format!("{}_WATERMARKED.{}", filename, extension),
                }
            } else {
                format!("{}_WATERMARKED.{}", filename, extension)
            };

            thread::spawn(move || {
                let x_padding: u32 = 20;
                let y_padding: u32 = processing_image.height() - x_padding;

                let wx_range = (
                    processing_image.width() - x_padding - processing_watermark.width(),
                    processing_image.width() - x_padding,
                );

                let wy_range = (
                    processing_image.height() - y_padding,
                    processing_image.height() - y_padding + processing_watermark.height(),
                );

                let mut wx: u32 = 0;
                let mut wy: u32 = 0;

                for (x, y, mut pixel) in processing_image.pixels() {
                    if x > wx_range.0 && x <= wx_range.1 && y >= wy_range.0 && y < wy_range.1 {
                        let w_pixel = processing_watermark.get_pixel(wx, wy);

                        if w_pixel.0[3] == 0 {
                            img.put_pixel(x, y, pixel);
                        } else {
                            // Appling Watermark pixel
                            pixel.blend(&w_pixel);
                            img.put_pixel(x, y, pixel);
                        }

                        wx += 1;

                        if wx == processing_watermark.width() {
                            wx = 0;
                            wy += 1;

                            if wy == processing_watermark.height() {
                                wy = 0;
                            }
                        }
                    } else {
                        img.put_pixel(x, y, pixel);
                    }

                    pb.inc(1);
                    pb.set_message(format!(
                        "{:3}%",
                        100 * pb.position() / ((img.width() * img.height()) as u64)
                    ));
                }

                match img.save_with_format(
                    path,
                    ACCEPTED_MIMETYPES
                        .iter()
                        .find(|m| m.extension == extension)
                        .expect("MimeType not supported")
                        .format,
                ) {
                    Ok(_) => pb.finish_with_message("100% <Operation completed successfully>"),
                    Err(_) => pb.finish_with_message("Operation Failed"),
                }
            })
        })
        .collect();

    for thread in handles {
        thread.join().expect("Thread Error");
    }
}
