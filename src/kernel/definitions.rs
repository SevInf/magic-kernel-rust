pub(crate) trait KernelDefinition {
    fn bounds(out: u32, scale_factor: f64) -> (u32, u32);
    fn weight(in_: u32, out: u32, scale_factor: f64) -> f64;
}

pub(crate) struct MagicKernel;

impl MagicKernel {
    const SUPPORT: f64 = 3.0;
}

impl KernelDefinition for MagicKernel {
    fn bounds(out: u32, scale_factor: f64) -> (u32, u32) {
        let center = kernel_resize_in_from_out(out, scale_factor);
        let distance = 0.5 * Self::SUPPORT / f64::min(scale_factor, 1.0);

        let first = (center - distance).floor() as u32;
        let last = (center + distance).ceil() as u32;
        (first, last)
    }

    fn weight(in_: u32, out: u32, scale_factor: f64) -> f64 {
        if scale_factor >= 1.0 {
            let center = kernel_resize_in_from_out(out, scale_factor);
            magic_kernel_value(in_ as f64 - center)
        } else {
            let center = kernel_resize_out_from_in(in_, scale_factor);
            magic_kernel_value(out as f64 - center)
        }
    }
}

fn magic_kernel_value(x: f64) -> f64 {
    if x <= -1.5 {
        0.0
    } else if x <= -0.5 {
        0.5 * (x + 1.5).powi(2)
    } else if x <= 0.5 {
        0.75 - x.powi(2)
    } else if x <= 1.5 {
        0.5 * (x - 1.5).powi(2)
    } else {
        0.0
    }
}

fn kernel_resize_out_from_in(in_: u32, scale_factor: f64) -> f64 {
    scale_factor * (in_ as f64 + 0.5) - 0.5
}

fn kernel_resize_in_from_out(out: u32, scale_factor: f64) -> f64 {
    (out as f64 + 0.5) / scale_factor - 0.5
}

pub struct Sharp2013;
impl KernelDefinition for Sharp2013 {
    fn weight(in_: u32, out: u32, _scale_factor: f64) -> f64 {
        let rel = out.abs_diff(in_);
        match rel {
            0 => 1.5,
            1 => -0.25,
            _ => panic!("Impossible rel"),
        }
    }

    fn bounds(out: u32, _scale_factor: f64) -> (u32, u32) {
        let min = if out == 0 { out } else { out - 1 };
        (min, out + 1)
    }
}

pub struct Sharp2021;
impl KernelDefinition for Sharp2021 {
    fn weight(in_: u32, out: u32, _scale_factor: f64) -> f64 {
        let rel = out.abs_diff(in_);
        match rel {
            0 => 17.0 / 18.0,
            1 => 0.0,
            2 => 1.0 / 36.0,
            _ => panic!("Impossible rel"),
        }
    }

    fn bounds(out: u32, _scale_factor: f64) -> (u32, u32) {
        let min = if out < 2 { out } else { out - 2 };
        (min, out + 2)
    }
}
