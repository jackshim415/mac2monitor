use core::slice;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
}

pub struct Framebuffer {
    ptr: *mut u8,
    size: usize,
    width: usize,
    height: usize,
    stride: usize,
}

impl Framebuffer {
    pub unsafe fn new(
        ptr: *mut u8,
        size: usize,
        width: usize,
        height: usize,
        stride: usize,
    ) -> Self {
        Self {
            ptr,
            size,
            width,
            height,
            stride,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = (y * self.stride + x) * 4;

        if offset + 3 >= self.size {
            return;
        }

        unsafe {
            let pixels = slice::from_raw_parts_mut(self.ptr, self.size);

            pixels[offset] = color.b;
            pixels[offset + 1] = color.g;
            pixels[offset + 2] = color.r;
            pixels[offset + 3] = 0;
        }
    }

    pub fn fill(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn fill_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        let x_end = (x + width).min(self.width);
        let y_end = (y + height).min(self.height);

        for py in y..y_end {
            for px in x..x_end {
                self.set_pixel(px, py, color);
            }
        }
    }
}