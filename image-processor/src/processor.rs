use crate::browser::alert;
use crate::utils::set_panic_hook;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Processor {}

#[wasm_bindgen]
impl Processor {

    pub fn test() {
        alert("this is the test function");
    }

    pub fn new() -> Self {
        set_panic_hook();
        Self{}
    }

    pub fn greet(&self) {
        alert("Hello, image-processor!");
    }
}
