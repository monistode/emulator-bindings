use wasm_bindgen::prelude::*;

use crate::processor::WasmProcessor;

pub mod stack;

#[wasm_bindgen]
pub enum ProcessorType {
    Stack,
}

pub fn create_processor(processor_type: ProcessorType, binary: &[u8]) -> Box<dyn WasmProcessor> {
    match processor_type {
        ProcessorType::Stack => {
            let mut wrapper = stack::StackProcessorWrapper::new();
            wrapper.load_executable(binary);
            Box::new(wrapper)
        }
    }
}
