use super::KernelDefinition;

pub(crate) trait ApplicationTarget {
    fn read(&self, n: u32) -> f64;
    fn write(&mut self, n: u32, value: f64);
}

pub(crate) struct Kernel1D {
    pub number_out: u32,
    // map from "in" point to ("out" point, weight);
    weights_map: Vec<Vec<(u32, f64)>>,
}

impl Kernel1D {
    pub fn new<D: KernelDefinition>(number_in: u32, number_out: u32) -> Self {
        let scale_factor = number_out as f64 / number_in as f64;
        let weights_map: Vec<_> = (0..number_out)
            .map(|out| {
                let (first, last) = D::bounds(out, scale_factor);
                let first = first.max(0);

                let last = last.min(number_in - 1);

                let mut weights = vec![0.0; number_in as usize];
                for in_ in first..=last {
                    weights[in_ as usize] += D::weight(in_, out, scale_factor)
                }
                store_weights(weights, first as usize, last as usize)
            })
            .collect();

        Self {
            number_out,
            weights_map,
        }
    }

    pub fn apply_to<T: ApplicationTarget>(&self, direction: &mut T) {
        for out in 0..self.number_out {
            let weights = &self.weights_map[out as usize];

            let sum: f64 = weights.iter().map(|(n, w)| direction.read(*n) * (*w)).sum();

            direction.write(out, sum)
        }
    }
}

fn store_weights(weights: Vec<f64>, min: usize, max: usize) -> Vec<(u32, f64)> {
    let w_slice = &weights[min..=max];
    let sum: f64 = w_slice.iter().sum();
    let norm = 1.0 / sum;

    w_slice
        .iter()
        .enumerate()
        .filter(|(_, weight)| **weight != 0.0)
        .map(|(i, weight)| ((i + min) as u32, *weight * norm))
        .collect()
}
