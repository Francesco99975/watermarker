use image::DynamicImage;

pub struct LoadedImage {
    pub data: DynamicImage,
    pub filename: String,
    pub ext: String,
}
