use std::ops::{Index, IndexMut};

use crate::kernel::ApplicationTarget;

/// Buffer storing the image.
/// All pixel values are expected to be f64.
pub struct ImageF64 {
    buffer: Vec<f64>,
    channels: u8,
    width: u32,
    height: u32,
}

impl ImageF64 {
    /// Creates new empty buffer with the specified number of channels,
    /// width and heights. All pixel values will be initialized as 0.0
    pub fn new_empty(channels: u8, width: u32, height: u32) -> Self {
        Self {
            buffer: vec![0.0; (channels as u32 * width * height) as usize],
            channels,
            width,
            height,
        }
    }

    /// Creates a new image from pre-defined buffer.
    pub fn new(buffer: Vec<f64>, channels: u8, width: u32, height: u32) -> Self {
        Self {
            buffer,
            channels,
            width,
            height,
        }
    }

    /// Returns number of channels in the image
    pub fn channels(&self) -> u8 {
        self.channels
    }

    /// Returns width of an image
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns height of an image
    pub fn height(&self) -> u32 {
        self.height
    }

    fn buffer_index(&self, (channel, x, y): (u8, u32, u32)) -> usize {
        let channels = self.channels as u32;
        (y * self.width * channels + x * channels + channel as u32) as usize
    }
}

impl From<ImageF64> for Vec<f64> {
    fn from(value: ImageF64) -> Self {
        value.buffer
    }
}

impl Index<(u8, u32, u32)> for ImageF64 {
    type Output = f64;

    fn index(&self, index: (u8, u32, u32)) -> &Self::Output {
        &self.buffer[self.buffer_index(index)]
    }
}

impl IndexMut<(u8, u32, u32)> for ImageF64 {
    fn index_mut(&mut self, index: (u8, u32, u32)) -> &mut Self::Output {
        let index = self.buffer_index(index);
        &mut self.buffer[index]
    }
}

pub(crate) struct Row<'a> {
    pub in_: &'a ImageF64,
    pub out: &'a mut ImageF64,
    pub channel: u8,
    pub y: u32,
}

impl<'a> ApplicationTarget for Row<'a> {
    fn read(&self, x: u32) -> f64 {
        self.in_[(self.channel, x, self.y)]
    }

    fn write(&mut self, x: u32, value: f64) {
        self.out[(self.channel, x, self.y)] = value
    }
}

pub(crate) struct Column<'a> {
    pub in_: &'a ImageF64,
    pub out: &'a mut ImageF64,
    pub channel: u8,
    pub x: u32,
}

impl<'a> ApplicationTarget for Column<'a> {
    fn read(&self, y: u32) -> f64 {
        self.in_[(self.channel, self.x, y)]
    }

    fn write(&mut self, y: u32, value: f64) {
        self.out[(self.channel, self.x, y)] = value
    }
}
