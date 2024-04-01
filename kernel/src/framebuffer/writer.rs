use super::utils::{Color, Pixel, Point};
use super::font::get_bitmap;

use bootloader_api::info::FrameBufferInfo;
use bootloader_api::info::PixelFormat;
use core::fmt;

const LINE_SPACING: usize = 1;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

const FONT_WIDTH: usize = 9;
const FONT_HEIGHT: usize = 15;
const LINE_HEIGHT: usize = FONT_HEIGHT + LINE_SPACING;

pub static mut FRAME_BUFFER_INTERNAL: FramebufferWriter = FramebufferWriter {
    info: None,
    buffer: None,
    x_pos: 0,
    y_pos: 0,
};

pub struct FramebufferWriter {
    pub info: Option<FrameBufferInfo>,
    pub buffer: Option<&'static mut [u8]>,
    x_pos: usize,
    y_pos: usize,
}

impl FramebufferWriter {
    pub fn info(&self) -> Option<FrameBufferInfo> {
        unsafe { FRAME_BUFFER_INTERNAL.info }
    }

    pub fn clear(&mut self) {
        self.buffer.as_mut().unwrap().fill(0);
    }

    fn set_pixel(&mut self, pixel: &Pixel) {
        let info = self.info().unwrap();
        let (x, y) = pixel.point().into();
        if (0..info.width).contains(&x) && (0..info.height).contains(&y) {
            let index = (y * info.stride + x) * info.bytes_per_pixel;
            let color = match info.pixel_format {
                PixelFormat::Rgb => [pixel.color().r(), pixel.color().g(), pixel.color().b(), 0],
                PixelFormat::Bgr => [pixel.color().b(), pixel.color().g(), pixel.color().r(), 0],
                PixelFormat::U8 => [pixel.color().r(), pixel.color().r(), pixel.color().r(), 0],
                other => panic!("pixel format {:?} not supported in logger", other),
            };
            self.buffer.as_mut().unwrap()[index..(index + info.bytes_per_pixel)]
                .copy_from_slice(&color[..info.bytes_per_pixel])
        }
    }

    fn newline(&mut self) {
        self.y_pos += LINE_HEIGHT;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    fn write_rendered_char(&mut self, bitmap: [[u8; 9]; 15]) {
        for (y, row) in bitmap.iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                let pixel = &Pixel::new(
                    Point::new(self.x_pos + x, self.y_pos + y),
                    Color::new(*byte * 255, *byte * 255, *byte * 255),
                );
                self.set_pixel(pixel)
            }
        }
        self.x_pos += FONT_WIDTH + LETTER_SPACING;
    }

    fn scroll(&mut self) {
        let info = self.info.unwrap();

        for i in 0..(info.height - LINE_HEIGHT) {
            let old = (i + LINE_HEIGHT) * info.stride;
            let new = i * info.stride;
            self.buffer.as_mut().unwrap().copy_within(
                (old * info.bytes_per_pixel)..((old + info.stride) * info.bytes_per_pixel),
                new * info.bytes_per_pixel,
            );
        }

        for i in (info.height - LINE_HEIGHT)..info.height {
            for j in 0..info.width {
                let index = (i * info.stride + j) * info.bytes_per_pixel;
                self.buffer.as_mut().unwrap()[index..(index + info.bytes_per_pixel)]
                    .copy_from_slice(&[0, 0, 0, 0])
            }
        }

        self.y_pos -= LINE_HEIGHT;
    }

    fn write_char(&mut self, c: char) {
        let info = self.info.unwrap();

        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + FONT_WIDTH;
                if new_xpos >= info.width {
                    self.newline();
                }
                let new_ypos = self.y_pos + LINE_HEIGHT;
                if new_ypos > info.height {
                    self.scroll()
                }
                self.write_rendered_char(get_bitmap(c).unwrap());
            }
        }
    }
}

unsafe impl Send for FramebufferWriter {}
unsafe impl Sync for FramebufferWriter {}

impl fmt::Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}