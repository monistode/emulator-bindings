use wasm_bindgen::prelude::*;

use crate::processor::WasmProcessor;

pub mod acc;
pub mod cisc;
pub mod risc;
pub mod stack;

#[wasm_bindgen]
#[derive(Clone)]
pub enum ProcessorType {
    Stack,
    Acc,
    Risc,
    Cisc,
}

pub fn create_processor(processor_type: ProcessorType) -> Box<dyn WasmProcessor> {
    match processor_type {
        ProcessorType::Stack => {
            let wrapper = stack::StackProcessorWrapper::new();
            Box::new(wrapper)
        }
        ProcessorType::Acc => {
            let wrapper = acc::AccProcessorWrapper::new();
            Box::new(wrapper)
        }
        ProcessorType::Risc => {
            let wrapper = risc::RiscProcessorWrapper::new();
            Box::new(wrapper)
        }
        ProcessorType::Cisc => {
            let wrapper = cisc::CiscProcessorWrapper::new();
            Box::new(wrapper)
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
            name: "Accumulator".to_string(),
            description: "Accumulator-based processor".to_string(),
            type_: ProcessorType::Acc,
        },
        ProcessorMetadata {
            name: "RISC".to_string(),
            description: "RISC processor".to_string(),
            type_: ProcessorType::Risc,
        },
        ProcessorMetadata {
            name: "CISC".to_string(),
            description: "CISC processor".to_string(),
            type_: ProcessorType::Cisc,
        },
    ]
}
