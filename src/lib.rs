#![feature(async_closure)]

mod utils;
use std::u8;

use crate::memory::{MemoryBlock, MemoryType};
use crate::processor::{WasmProcessor, WasmProcessorContinue, WasmProcessorEnum};
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
    processor: WasmProcessorEnum,
}

#[wasm_bindgen]
impl Runner {
    #[wasm_bindgen(constructor)]
    pub fn new(processor_type: ProcessorType) -> Self {
        let processor = create_processor(processor_type);
        Runner { processor }
    }

    #[wasm_bindgen]
    pub fn load_program(&mut self, program: &[u8]) -> Result<(), String> {
        self.processor.load_executable(program)
    }

    #[wasm_bindgen]
    pub async fn run(
        &mut self,
        output: &js_sys::Function,
        input: &js_sys::Function,
    ) -> WasmProcessorContinue {
        let processor = unsafe { std::mem::transmute::<_, &'static mut WasmProcessorEnum>(self) };
        let output = unsafe { std::mem::transmute::<_, &'static js_sys::Function>(output) };
        let input = unsafe { std::mem::transmute::<_, &'static js_sys::Function>(input) };
        let value = processor.run(output, input);
        let pinned = std::pin::Pin::from(value);
        pinned.await
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

    #[wasm_bindgen]
    pub fn peek_stack(&mut self, n: u8) -> u16 {
        self.processor.peek_stack(n)
    }
}
