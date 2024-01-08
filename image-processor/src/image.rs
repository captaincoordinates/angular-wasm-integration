use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Self{width, height, pixels}
    }

    pub fn pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn pixels_count(&self) -> usize {
        self.pixels.len()
    }
}
