use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

use bootloader_api::info::FrameBuffer;
use bootloader_api::info::PixelFormat;
use spin::Mutex;

const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size24;
const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
const BACKUP_CHAR: char = '�';
const FONT_WEIGHT: FontWeight = FontWeight::Regular;

const LINE_SPACING: usize = 0;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 3;

const LINE_HEIGHT: usize = LINE_SPACING + CHAR_RASTER_HEIGHT.val();
const FONT_WIDTH: usize = CHAR_RASTER_WIDTH + LETTER_SPACING;

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub static FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer {
    buffer: None,
    x_pos: BORDER_PADDING,
    y_pos: BORDER_PADDING,
    width: 0,
    height: 0,
    pixel_format: PixelFormat::Rgb,
    bytes_per_pixel: 0,
    stride: 0,
});

pub struct Framebuffer {
    buffer: Option<&'static mut [u8]>,
    x_pos: usize,
    y_pos: usize,
    width: usize,
    height: usize,
    pixel_format: PixelFormat,
    bytes_per_pixel: usize,
    stride: usize,
}

impl Framebuffer {
    pub fn init(&mut self, framebuffer: &'static mut FrameBuffer) {
        let info = framebuffer.info();
        self.width = info.width;
        self.height = info.height;
        self.pixel_format = info.pixel_format;
        self.bytes_per_pixel = info.bytes_per_pixel;
        self.stride = info.stride;
        self.buffer = Some(framebuffer.buffer_mut());
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.buffer.as_mut().unwrap().fill(0);
    }

    fn newline(&mut self) {
        self.y_pos += LINE_HEIGHT;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn back(&mut self) {
        self.x_pos -= FONT_WIDTH;
    }

    pub fn forward(&mut self) {
        self.x_pos += FONT_WIDTH;
    }

    fn scroll(&mut self) {
        let lines = self.height / LINE_HEIGHT * LINE_HEIGHT;
        let src_start = LINE_HEIGHT * self.stride * self.bytes_per_pixel;
        let bytes_to_copy = (lines - LINE_HEIGHT) * self.stride * self.bytes_per_pixel;

        unsafe {
            core::ptr::copy(
                self.buffer.as_mut().unwrap()[src_start..].as_ptr(),
                self.buffer.as_mut().unwrap().as_mut_ptr(),
                bytes_to_copy,
            );
        }

        let clear_start = bytes_to_copy;
        self.buffer.as_mut().unwrap()[clear_start..].fill(0);
        self.y_pos = self.y_pos.saturating_sub(LINE_HEIGHT);
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + FONT_WIDTH;
                if new_xpos >= self.width {
                    self.newline();
                }

                let new_ypos = self.y_pos + LINE_HEIGHT;
                if new_ypos > self.height {
                    //self.y_pos = BORDER_PADDING;
                    self.scroll()
                }

                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.set_pixel(self.x_pos + x, self.y_pos + y, *byte, *byte, *byte);
            }
        }
        self.x_pos += FONT_WIDTH;
    }

    fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            let index = (y * self.stride + x) * self.bytes_per_pixel;
            let color = match self.pixel_format {
                PixelFormat::Rgb => [r, g, b, 0],
                PixelFormat::Bgr => [b, g, r, 0],
                PixelFormat::U8 => [r, r, r, 0],
                other => panic!("pixel format {:?} not supported in logger", other),
            };
            self.buffer.as_mut().unwrap()[index..(index + self.bytes_per_pixel)]
                .copy_from_slice(&color[..self.bytes_per_pixel])
        }
    }
}

unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}

impl core::fmt::Write for Framebuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
