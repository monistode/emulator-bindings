use wasm_bindgen::prelude::*;

use crate::processor::WasmProcessor;

pub mod cisc;
pub mod stack;

#[wasm_bindgen]
#[derive(Clone)]
pub enum ProcessorType {
    Stack,
    Cisc,
}

pub fn create_processor(
    processor_type: ProcessorType,
    binary: &[u8],
) -> (Box<dyn WasmProcessor>, Result<(), String>) {
    match processor_type {
        ProcessorType::Stack => {
            let mut wrapper = stack::StackProcessorWrapper::new();
            let result = wrapper.load_executable(binary);
            (Box::new(wrapper), result)
        }
        ProcessorType::Cisc => {
            let mut wrapper = cisc::CiscProcessorWrapper::new();
            let result = wrapper.load_executable(binary);
            (Box::new(wrapper), result)
        }
    }
}

#[wasm_bindgen]
pub struct ProcessorMetadata {
    name: String,
    description: String,
    type_: ProcessorType,
}

#[wasm_bindgen]
impl ProcessorMetadata {
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[wasm_bindgen]
    pub fn type_(&self) -> ProcessorType {
        self.type_.clone()
    }
}

#[wasm_bindgen]
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
