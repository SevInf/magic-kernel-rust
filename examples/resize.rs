use image::{
    codecs::jpeg::JpegEncoder, io::Reader as ImageReader, DynamicImage, Rgb32FImage, RgbImage,
};
use magic_kernel::{magic_resize, ImageF64, Version};

pub fn main() {
    let image = ImageReader::open("example.jpg").unwrap().decode().unwrap();
    let resized = magic_resize(
        &convert_from_image_rs(image),
        Version::MagicKernelSharp2021,
        None,
        Some(500),
    );

    let writer = std::fs::File::create("out.jpg").unwrap();
    let mut encoder = JpegEncoder::new_with_quality(writer, 85);
    encoder.encode_image(&convert_to_image_rs(resized)).unwrap();
}

fn convert_from_image_rs(image: DynamicImage) -> ImageF64 {
    let img_f32 = image.to_rgb32f();
    let samples_layout = img_f32.sample_layout();
    let data = img_f32.to_vec();

    ImageF64::new(
        data.into_iter().map(|val| val as f64).collect(),
        samples_layout.channels,
        samples_layout.width,
        samples_layout.height,
    )
}

fn convert_to_image_rs(image: ImageF64) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let buffer: Vec<_> = image.into();

    let f32_image = Rgb32FImage::from_vec(
        width,
        height,
        buffer.into_iter().map(|x| x as f32).collect(),
    )
    .unwrap();

    DynamicImage::ImageRgb32F(f32_image).to_rgb8()
}
