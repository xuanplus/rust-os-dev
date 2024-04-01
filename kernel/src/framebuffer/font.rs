// pixels: 144x90; characters: 16x5; szie: 9x15
const FONT_RAW: &[u8; 1620] = include_bytes!("./font_9x15.raw");
static mut BITMAP: Option<Bitmap> = None;

struct Bitmap {
    pub map: [[u8; 144]; 90]
}

pub fn bitmap_init() {
    let mut result: [[u8; 144]; 90] = [[0; 144]; 90];

    for (index, value) in FONT_RAW.iter().enumerate() {
        let row = index / 18;
        let col = index % 18 * 8;
        
        for i in 0..8 {
            result[row][col + 7 - i] = (value >> i) & 1;
        }
    }

    unsafe {
        BITMAP = Some(Bitmap { map: result })
    }
}

pub fn get_bitmap(c: char) -> Option<[[u8; 9]; 15]> {
    let i = c as usize;

    if (32..127).contains(&i) {
        let i = i - 32;

        let row = i / 16 * 15;
        let col = i % 16 * 9;

        let mut result: [[u8; 9]; 15] = [[0; 9]; 15];

        for i in row..(row + 15) {
            result[i - row].copy_from_slice(unsafe {
                let bitmap = BITMAP.as_ref().unwrap();
                &bitmap.map[i][col..(col + 9)]
            })
        }

        Some(result)
    } else {
        crate::print!("No ascii character: u8({})", i);
        None
    }
}