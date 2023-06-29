use std::{env, fs, process};

use constants::ACCEPTED_MIMETYPES;
use helpers::get_dirs_images_paths;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use structures::LoadedImage;

mod constants;
mod helpers;
mod structures;

async fn compute_image(
    mut processing_image: DynamicImage,
    processing_watermark: DynamicImage,
    wx_range: (u32, u32),
    wy_range: (u32, u32),
    pb: &ProgressBar,
    path: String,
    extension: String,
) {
    let mut wx: u32 = 0;
    let mut wy: u32 = 0;

    for y in wy_range.0..wy_range.1 {
        for x in wx_range.0..wx_range.1 {
            let mut pixel = processing_image.get_pixel(x, y);
            let w_pixel = processing_watermark.get_pixel(wx, wy);

            // Appling Watermark pixel
            // println!("Appling Watermark pixel: {} {}", wx, wy);
            pixel.blend(&w_pixel);
            processing_image.put_pixel(x, y, pixel);

            wx += 1;

            if wx == processing_watermark.width() {
                wx = 0;
                wy += 1;

                if wy == processing_watermark.height() {
                    wy = 0;
                }
            }

            pb.inc(1);
            pb.set_message(format!(
                "{:3}%",
                100 * pb.position()
                    / ((processing_watermark.width() * processing_watermark.height()) as u64)
            ));
        }
    }

    match processing_image.save_with_format(
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
}

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
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

    let styles: [&str; 5] = ["█  ", "█▉▊▋▌▍▎▏  ", "█▇▆▅▄▃▂▁  ", "█▓▒░  ", "█▛▌▖  "];

    let colors: [&str; 7] = [
        "red", "yellow", "green", "blue", "magenta", "orange", "purple",
    ];

    let multi_bars = MultiProgress::new();

    let handles: Vec<_> = images_to_process
        .iter()
        .map(|image_to_process| {
            let pb = multi_bars.add(ProgressBar::new(
                (watermark.width() * watermark.height()) as u64,
            ));
            let syl = styles
                .choose(&mut rand::thread_rng())
                .expect("Could not choose style");
            let col = colors
                .choose(&mut rand::thread_rng())
                .expect("Could not choose color");
            pb.set_style(
                ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", col))
                    .unwrap()
                    .progress_chars(syl),
            );
            pb.set_prefix(format!(
                "Appling Watermark for {}.{}",
                image_to_process.filename, image_to_process.ext,
            ));

            let processing_image = image_to_process.data.clone();
            let processing_watermark = watermark.clone();
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

            tokio::spawn(async move {
                compute_image(
                    processing_image,
                    processing_watermark,
                    wx_range,
                    wy_range,
                    &pb,
                    path,
                    extension,
                )
                .await
            })
        })
        .collect();

    for thread in handles {
        thread.await.unwrap();
    }
}
