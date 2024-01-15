use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = appLogger, js_name = wasmLog)]
    pub fn browser_log(s: &str); 
}
