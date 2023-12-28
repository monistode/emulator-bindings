mod utils;
use std::u8;

use crate::memory::{MemoryBlock, MemoryType};
use crate::processor::{WasmProcessor, WasmProcessorContinue};
use processors::{create_processor, ProcessorType};
use registers::RegisterState;
use wasm_bindgen::prelude::*;

pub use processors::available_processors;
mod memory;
mod processor;
mod processors;
mod registers;

#[wasm_bindgen]
pub struct Runner {
    processor: Box<dyn WasmProcessor>,
}

#[wasm_bindgen]
impl Runner {
    #[wasm_bindgen(constructor)]
    pub fn new(processor_type: ProcessorType, binary: &[u8]) -> Self {
        Runner {
            processor: create_processor(processor_type, binary),
        }
    }

    #[wasm_bindgen]
    pub fn run(
        &mut self,
        output: &js_sys::Function,
        input: &js_sys::Function,
    ) -> WasmProcessorContinue {
        self.processor.run(output, input)
    }

    #[wasm_bindgen]
    pub fn run_n(
        &mut self,
        output: &js_sys::Function,
        input: &js_sys::Function,
        n: usize,
    ) -> WasmProcessorContinue {
        self.processor.run_n(output, input, n)
    }

    #[wasm_bindgen]
    pub fn get_memory(&mut self) -> Vec<MemoryBlock> {
        self.processor.get_memory()
    }

    #[wasm_bindgen]
    pub fn set_memory(&mut self, mem_type: MemoryType, index: usize, value: u8) -> bool {
        self.processor.set_memory(mem_type, index, value)
    }

    #[wasm_bindgen]
    pub fn get_registers(&mut self) -> Vec<RegisterState> {
        self.processor.get_registers()
    }
}
