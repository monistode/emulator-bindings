use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RegisterState {
    name: String,
    value: u16,
}

#[wasm_bindgen]
impl RegisterState {
    pub fn new(name: String, value: u16) -> Self {
        RegisterState { name, value }
    }

    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen]
    pub fn value(&self) -> u16 {
        self.value
    }
}
