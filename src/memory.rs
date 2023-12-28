use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum MemoryType {
    Text,
    Data,
}

#[wasm_bindgen]
pub struct MemoryBlock {
    pub memory_type: MemoryType,
    #[wasm_bindgen(skip)]
    pub values: Vec<u8>,
}

#[wasm_bindgen]
impl MemoryBlock {
    #[wasm_bindgen]
    pub fn values(&self) -> Vec<u8> {
        self.values.clone()
    }

    #[wasm_bindgen]
    pub fn cell_type(&self) -> MemoryType {
        self.memory_type
    }

    #[wasm_bindgen]
    pub fn cell_type_name(&self) -> String {
        match self.memory_type {
            MemoryType::Text => "text".to_string(),
            MemoryType::Data => "data".to_string(),
        }
    }
}
