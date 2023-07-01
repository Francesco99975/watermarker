use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use indicatif::ProgressBar;

use crate::constants::ACCEPTED_MIMETYPES;

pub async fn compute_image(
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
