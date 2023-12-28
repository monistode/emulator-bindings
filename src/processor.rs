use crate::{
    memory::{MemoryBlock, MemoryType},
    registers::RegisterState,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum WasmProcessorContinue {
    Continue,
    Halt,
}

pub trait WasmProcessor {
    fn run(&mut self, output: &js_sys::Function, input: &js_sys::Function)
        -> WasmProcessorContinue;
    fn run_n(
        &mut self,
        output: &js_sys::Function,
        input: &js_sys::Function,
        n: usize,
    ) -> WasmProcessorContinue;
    fn get_memory(&mut self) -> Vec<MemoryBlock>;
    fn set_memory(&mut self, mem_type: MemoryType, index: usize, value: u8) -> bool;
    fn get_registers(&mut self) -> Vec<RegisterState>;
    fn load_executable(&mut self, binary: &[u8]) -> Result<(), String>;
}
