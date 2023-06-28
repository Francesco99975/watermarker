use image::ImageFormat;

pub struct MimeType<'a> {
    pub extension: &'a str,
    pub format: ImageFormat,
}

pub const ACCEPTED_MIMETYPES: [MimeType; 5] = [
    MimeType {
        extension: "png",
        format: ImageFormat::Png,
    },
    MimeType {
        extension: "jpeg",
        format: ImageFormat::Jpeg,
    },
    MimeType {
        extension: "jpg",
        format: ImageFormat::Jpeg,
    },
    MimeType {
        extension: "webp",
        format: ImageFormat::WebP,
    },
    MimeType {
        extension: "bmp",
        format: ImageFormat::Bmp,
    },
];
