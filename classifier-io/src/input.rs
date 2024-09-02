use image::ImageError;
use ndarray::prelude::*;
use rgb::{FromSlice, Gray};

/// Load and prepare an input image for inference.
///   - Decode image (JPEG or PNG).
///   - Resize to `width` x `height`.
///   - Convert to grayscale.
///   - Invert luminance.
pub fn prepare_input_image(
    image_file_content: &[u8],
    width: usize,
    height: usize,
) -> Result<Array2<f32>, ImageError> {
    let arr_u8 = load_resize_luma_u8(image_file_content, width, height)?;
    let arr_f32 = arr_u8.mapv(|x| x.0 as f32 / 255.0);
    let inverted = arr_f32.mapv(|x| 1.0 - x); // Invert luminance
    Ok(inverted)
}

fn load_resize_luma_u8(
    src_bytes: &[u8],
    width: usize,
    height: usize,
) -> Result<Array2<Gray<u8>>, ImageError> {
    let src = image::load_from_memory(src_bytes)?.into_luma8();
    let (src_width, src_height) = (src.width() as usize, src.height() as usize);
    let src = src.into_vec();
    let mut dst = vec![Gray::new(0u8); width * height];
    let mut resizer = resize::new(
        src_width,
        src_height,
        width,
        height,
        resize::Pixel::Gray8,
        resize::Type::Point,
    )
    .unwrap();
    resizer.resize(src.as_gray(), &mut dst).unwrap();
    Ok(Array2::from_shape_vec((height, width), dst).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: usize = 28;
    const H: usize = 28;

    #[test]
    fn load_jpeg() {
        let src = include_bytes!("test-fixtures/example.jpg");
        let input = prepare_input_image(src, W, H).unwrap();
        assert_eq!(input.shape(), &[W, H]);
    }

    #[test]
    fn load_png() {
        let src = include_bytes!("test-fixtures/example.png");
        let input = prepare_input_image(src, W, H).unwrap();
        assert_eq!(input.shape(), &[W, H]);
    }
}
