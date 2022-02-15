mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn rand_u32(range: u32) -> u32 {
    return ( random() * (range as f64)) as u32;
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new (width: u32, height: u32) -> Universe {
        let buffersize = width * height * 4;
        let buffer = (0..buffersize)
                .map(|_| {0})
                .collect();
        
        Universe {
            width,
            height,
            buffer,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }


    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn buffer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn fillbox(&mut self,color: u32){
        let height = self.height;
        let width = self.width;
        let buf = &mut self.buffer;
        // Color model u32 LE (RGBA)  -> u8 BGRA
        let blue: u8 = ((color  >> 16) & 0xff)  as u8; // R = 1.0
        let green: u8  = ((color >> 8) & 0xff) as u8; // G = 1.0
        let red: u8 = ((color >> 0) & 0xff) as u8; // B = 1.0
        let alpha: u8 = 0xff;

        log(&format!("{} {} {}",blue,green,red));

        for y  in 0..height {
            let offset = y * width * 4;
            for x in 0..width {
                let pos :usize = (offset + x * 4) as usize;
                buf[pos] = blue;
                buf[pos + 1] = green;
                buf[pos + 2] = red;
                buf[pos + 3] = alpha;
            }
        }
    }
    
    pub fn fillrandomrect(&mut self){
        let height = self.height;
        let width = self.width;
        let buf = &mut self.buffer;

        let startx:u32 = rand_u32(width);
        let starty:u32 = rand_u32(height);
        let endx:u32 = rand_u32(width-startx); 
        let endy:u32 = rand_u32(height-starty);
        let red:u8 = rand_u32(255) as u8;
        let green:u8 = rand_u32(255) as u8;
        let blue:u8 = rand_u32(255) as u8;
        let alpha:u8 = rand_u32(255) as u8;

        for y in starty..endy {
            let offset = y * width * 4;
            for x  in startx..endx {
                let pos :usize= (offset + (x * 4)) as usize;

                buf[pos] = blue;
                buf[pos + 1] = green;
                buf[pos + 2] = red;
                buf[pos + 3] = alpha;
            }
        }
    }
}
