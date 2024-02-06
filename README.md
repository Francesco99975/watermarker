# [Watermarker](https://github.com/Francesco99975/watermarker/releases/download/0.3.0/watermarker) - Image Marker

### CLI program that takes images as arguments and counts the last image as a watermark to be applied to other images.

---

## **Install**

```bash
cargo install watermarker
```

### Or Download the latest [release](https://github.com/Francesco99975/watermarker/releases/download/0.3.0/watermarker)

---

## **Usage**

```bash
watermarker [OPTION_DIRS] [IMAGES_PATHS/DIRS_PATHS...] [WATERMARK_PATH] [OPTION_OUTPUT] [OUTPUT_PATH]
```

_Simple Example_

```bash
watermarker image1.png watermark.png
```

_Different paths Example_

```bash
watermarker ./image1.png ./photos/image2.png ../image3.png watermark.png
```

_Output directory for images list_

```bash
watermarker image1.png image2.png  watermark.png -o results
```

_Directory lists Example_

```bash
watermarker -d dir1/imgs_dir photos_dir watermark.png
```

_Output directory for directories list_

```bash
watermarker -d dir1/imgs_dir photos_dir watermark.png -o results
```
