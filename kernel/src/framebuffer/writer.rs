use self::font_constants::LINE_HEIGHT;

use super::utils::{Color, Pixel, Point};

use bootloader_api::info::FrameBufferInfo;
use bootloader_api::info::PixelFormat;
use core::fmt;
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

pub static mut FRAME_BUFFER_INTERNAL: FramebufferWriter = FramebufferWriter {
    info: None,
    buffer: None,
    x_pos: 0,
    y_pos: 0,
};

const LINE_SPACING: usize = 0;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

mod font_constants {
    use super::*;
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size20;
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
    pub const LINE_HEIGHT: usize = CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
    pub const BACKUP_CHAR: char = '�';
}

pub struct FramebufferWriter {
    pub info: Option<FrameBufferInfo>,
    pub buffer: Option<&'static mut [u8]>,
    x_pos: usize,
    y_pos: usize,
}

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
            c,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
    }
    get(c).unwrap_or_else(|| {
        get(font_constants::BACKUP_CHAR).expect("Should get raster of backup char.")
    })
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
                PixelFormat::Rgb => [pixel.color().r(), pixel.color().g(), pixel.color().b()],
                PixelFormat::Bgr => [pixel.color().b(), pixel.color().g(), pixel.color().r()],
                PixelFormat::U8 => [pixel.color().r(), pixel.color().r(), pixel.color().r()],
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

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                let pixel = &Pixel::new(
                    Point::new(self.x_pos + x, self.y_pos + y),
                    Color::new(*byte, *byte, *byte),
                );
                self.set_pixel(pixel)
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn scroll(&mut self) {
        let info = self.info.unwrap();

        for i in 0..(info.height - LINE_HEIGHT * 2) {
            let old = (i + LINE_HEIGHT) * info.stride;
            let new = i * info.stride;
            self.buffer.as_mut().unwrap().copy_within(
                (old * info.bytes_per_pixel)..((old + info.stride) * info.bytes_per_pixel),
                new * info.bytes_per_pixel,
            );
        }

        for i in (info.height - (LINE_HEIGHT * 2))..(info.height - LINE_HEIGHT) {
            for j in 0..info.width {
                let index = (i * info.stride + j) * info.bytes_per_pixel;
                self.buffer.as_mut().unwrap()[index..(index + info.bytes_per_pixel)]
                    .copy_from_slice(&[0, 0, 0])
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
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= info.width {
                    self.newline();
                }
                let new_ypos = self.y_pos + LINE_HEIGHT;
                if new_ypos >= info.height {
                    self.scroll()
                }
                self.write_rendered_char(get_char_raster(c));
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
