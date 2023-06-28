use std::ops::{Bound, RangeBounds};
use std::{env, process};

use image::{GenericImageView, ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};

const ACCEPTED_MIMETYPES: [&str; 5] = ["png", "jpeg", "ico", "webp", "bmp"];

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Please enter the path to 2 images");
        process::exit(1);
    }

    let img_path = &args[1];
    let watermark_path = &args[2];

    let dot_index_1 = match img_path.chars().skip(1).position(|x| x == '.') {
        Some(inx) => inx,
        None => process::exit(1),
    };

    let dot_index_2 = match watermark_path.chars().skip(1).position(|x| x == '.') {
        Some(inx) => inx,
        None => process::exit(1),
    };

    let mime1 = img_path.substring(dot_index_1 + 2, img_path.len());
    let mime2 = watermark_path.substring(dot_index_2 + 2, watermark_path.len());

    if !ACCEPTED_MIMETYPES.contains(&mime1) || !ACCEPTED_MIMETYPES.contains(&mime2) {
        println!("{}", mime1);
        println!("{}", mime2);
        println!("Did not find valid images");
        process::exit(1)
    }

    let main_img = image::open(img_path).expect("File not found");
    let watermark = image::open(watermark_path).expect("File not found");

    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(main_img.width(), main_img.height());

    // let styles = [
    //     ("Rough bar:", "█  ", "red"),
    //     ("Fine bar: ", "█▉▊▋▌▍▎▏  ", "yellow"),
    //     ("Vertical: ", "█▇▆▅▄▃▂▁  ", "green"),
    //     ("Fade in:  ", "█▓▒░  ", "blue"),
    //     ("Blocky:   ", "█▛▌▖  ", "magenta"),
    // ];

    let pb = ProgressBar::new((img.width() * img.height()) as u64);
    pb.set_style(
        ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", "yellow"))
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
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

    for (x, y, pixel) in main_img.pixels() {
        if x > wx_range.0 && x <= wx_range.1 && y >= wy_range.0 && y < wy_range.1 {
            let w_pixel = watermark.get_pixel(wx, wy);

            if w_pixel.0[3] == 0 {
                img.put_pixel(x, y, image::Rgba(pixel.0.map(|p| p)));
            } else {
                // println!("Writing watermark pixel");
                img.put_pixel(x, y, image::Rgba(w_pixel.0.map(|p| p)));
            }

            wx += 1;

            if wx == watermark.width() {
                wx = 0;
                wy += 1;

                if wy == watermark.height() {
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

    match img.save_with_format("./results/output.png", image::ImageFormat::Png) {
        Ok(_) => pb.finish_with_message("100% <Operation completed successfully>"),
        Err(_) => pb.finish_with_message("Operation Failed"),
    }
}
