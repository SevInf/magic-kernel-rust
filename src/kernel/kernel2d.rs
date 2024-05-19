use crate::imagef64::{Column, ImageF64, Row};

use super::{kernel1d::Kernel1D, KernelDefinition};

pub(crate) struct Kernel2D {
    x: Kernel1D,
    y: Kernel1D,
}

impl Kernel2D {
    pub fn new<Def: KernelDefinition>(from_size: (u32, u32), to_size: (u32, u32)) -> Self {
        Self {
            x: Kernel1D::new::<Def>(from_size.0, to_size.0),
            y: Kernel1D::new::<Def>(from_size.1, to_size.1),
        }
    }

    pub fn apply(&self, image: &ImageF64) -> ImageF64 {
        let mut tmp = ImageF64::new_empty(image.channels(), self.x.number_out, image.height());
        for channel in 0..image.channels() {
            for y in 0..image.height() {
                let mut row = Row {
                    in_: image,
                    out: &mut tmp,
                    channel,
                    y,
                };

                self.x.apply_to(&mut row);
            }
        }

        let mut out = ImageF64::new_empty(image.channels(), self.x.number_out, self.y.number_out);
        for channel in 0..image.channels() {
            for x in 0..tmp.width() {
                let mut column = Column {
                    in_: &mut tmp,
                    out: &mut out,
                    channel,
                    x,
                };

                self.y.apply_to(&mut column);
            }
        }

        out
    }
}
