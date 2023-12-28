use wasm_bindgen::prelude::*;

use crate::processor::WasmProcessor;

pub mod cisc;
pub mod stack;

#[wasm_bindgen]
pub enum ProcessorType {
    Stack,
    Cisc,
}

pub fn create_processor(processor_type: ProcessorType, binary: &[u8]) -> Box<dyn WasmProcessor> {
    match processor_type {
        ProcessorType::Stack => {
            let mut wrapper = stack::StackProcessorWrapper::new();
            wrapper.load_executable(binary);
            Box::new(wrapper)
        }
        ProcessorType::Cisc => {
            let mut wrapper = cisc::CiscProcessorWrapper::new();
            wrapper.load_executable(binary);
            Box::new(wrapper)
        }
    }
}

pub struct ProcessorMetadata {
    pub name: String,
    pub description: String,
    pub type_: ProcessorType,
}

pub fn available_processors() -> Vec<ProcessorMetadata> {
    vec![
        ProcessorMetadata {
            name: "Stack".to_string(),
            description: "Stack-based processor".to_string(),
            type_: ProcessorType::Stack,
        },
        ProcessorMetadata {
            name: "CISC".to_string(),
            description: "CISC processor".to_string(),
            type_: ProcessorType::Cisc,
        },
    ]
}
