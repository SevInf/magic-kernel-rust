use crate::{
    kernel::{Kernel2D, MagicKernel, Sharp2013, Sharp2021},
    ImageF64,
};

/// Version of a resize algorithm.
/// See <https://johncostella.com/magic/> for details and difference.
#[derive(PartialEq, Eq)]
pub enum Version {
    /// Original magic kernel version with no extra sharpening
    MagicKernel,
    /// 2013 version of Magic Kernel Sharp
    MagicKernelSharp2013,
    /// 2021 version of Magic Kernel Sharp
    MagicKernelSharp2021,
}

/// Resizes image to a new size. Any of the dimensions could be optional - in that
/// case, image would be scaled proportionally.
///
/// # Examples
///
/// ```no_run
/// // set width, scale height proportionally
/// use magic_kernel::{Version, magic_resize};
/// # let image = magic_kernel::ImageF64::new_empty(0, 0, 0);
/// let result = magic_resize(&image, Version::MagicKernelSharp2021, Some(500), None);
/// ```
/// ```no_run
/// // set height, scale width proportionally
/// use magic_kernel::{Version, magic_resize};
/// # let image = magic_kernel::ImageF64::new_empty(0, 0, 0);
/// let result = magic_resize(&image, Version::MagicKernelSharp2021, None, Some(500));
/// ```
/// ```no_run
/// // scale both width and height
/// use magic_kernel::{Version, magic_resize};
/// # let image = magic_kernel::ImageF64::new_empty(0, 0, 0);
/// let result = magic_resize(&image, Version::MagicKernelSharp2021, Some(250), Some(500));
/// ```
pub fn magic_resize(
    image: &ImageF64,
    version: Version,
    new_width: Option<u32>,
    new_height: Option<u32>,
) -> ImageF64 {
    let new_size = match (new_width, new_height) {
        (Some(new_width), Some(new_height)) => (new_width, new_height),
        (Some(new_width), None) => {
            let factor = new_width as f64 / image.width() as f64;
            let new_height = (image.height() as f64 * factor) as u32;
            (new_width, new_height)
        }

        (None, Some(new_height)) => {
            let factor = new_height as f64 / image.height() as f64;
            let new_width = (image.width() as f64 * factor) as u32;
            (new_width, new_height)
        }
        (None, None) => (image.width(), image.height()),
    };

    let size = (image.width(), image.height());

    let out = Kernel2D::new::<MagicKernel>(size, new_size).apply(image);
    if version == Version::MagicKernel {
        return out;
    }
    let out = Kernel2D::new::<Sharp2013>(new_size, new_size).apply(&out);
    if version == Version::MagicKernelSharp2013 {
        return out;
    }

    // Sharp 2021 version
    Kernel2D::new::<Sharp2021>(new_size, new_size).apply(&out)
}
