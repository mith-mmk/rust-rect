mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
const BUFFER_SIZE: usize = 2000 * 2000 * 4;
static BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
pub fn get_buffer() -> *const u8 {
    return &BUFFER[0];
 }

fn rand_u32(range: u32) -> u32 {
    return ( random() * (range as f64)) as u32;
}

#[wasm_bindgen]
pub fn fillbox(buf: &mut [u8],width: u32,height: u32,color: u32){
    let blue: u8 = (color & 0xff) as u8; // B = 1.0
    let green: u8  = ((color >> 8) & 0xff) as u8; // G = 1.0
    let red: u8 = ((color >> 16) & 0xff) as u8; // R = 1.0
    let alpha: u8 = 0xff;
    for y  in 0..height {
        let offset = y * width;
        for x in 0..width {
            let pos = offset + x * 4;
            unsafe {
                buf[pos as usize] = blue;
                buf[(pos + 1) as usize] = green;
                buf[(pos + 2) as usize] = red;
                buf[(pos + 3) as usize] = alpha;
            }
        }
    }
}

#[wasm_bindgen]
pub fn fillrandomrect(buf: &mut [u8],width: u32,height: u32){
    let startx:u32 = rand_u32(width);
    let starty:u32 = rand_u32(height);
    let endx:u32 = rand_u32((width-startx) + 1); 
    let endy:u32 = rand_u32((height-starty) + 1);
    let red:u8 = rand_u32(255) as u8;
    let green:u8 = rand_u32(255) as u8;
    let blue:u8 = rand_u32(255) as u8;
    let alpha:u8 = rand_u32(255) as u8;

    for y in starty..endy {
        let offset = y * width;
        for x  in startx..endx {
            let pos = offset + (x * 4);
            unsafe {
                buf[pos as usize] = blue;
                buf[(pos + 1) as usize] = green;
                buf[(pos + 2) as usize] = red;
                buf[(pos + 3) as usize] = alpha;
            }
        }
    }
}