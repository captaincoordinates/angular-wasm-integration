use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(s: &str); 
}
