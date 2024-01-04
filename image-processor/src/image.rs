use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Image {
    pub width: u16,
    pub height: u16,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>) -> Self {
        Self{width, height, pixels}
    }

    pub fn pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}
