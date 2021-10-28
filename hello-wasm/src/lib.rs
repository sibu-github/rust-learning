use wasm_bindgen::prelude::*;
use web_sys::console;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
    console::log_1(&format!("Hello, {}!", name).into());
    
}



