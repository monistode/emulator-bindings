use wasm_bindgen::prelude::*;

use crate::processor::WasmProcessor;
use crate::registers::RegisterState;
use crate::{MemoryBlock, MemoryType};
use monistode_emulator::cisc_processor;
use monistode_emulator::common::Processor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct CiscProcessorWrapper {
    processor: cisc_processor::CiscProcessor,
}

impl CiscProcessorWrapper {
    pub fn new() -> Self {
        CiscProcessorWrapper {
            processor: cisc_processor::CiscProcessor::new(),
        }
    }
}

impl WasmProcessor for CiscProcessorWrapper {
    type ProcessorType = cisc_processor::CiscProcessor;
    type InstructionType = u8;

    fn get_processor(&self) -> &Self::ProcessorType {
        &self.processor
    }

    fn get_processor_mut(&mut self) -> &mut Self::ProcessorType {
        &mut self.processor
    }

    fn get_memory(&mut self) -> Vec<MemoryBlock> {
        let mut result = Vec::new();

        let mut memory = Vec::new();
        for value in self.processor.memory.memory.iter() {
            memory.push((*value).into());
        }
        result.push(MemoryBlock {
            memory_type: MemoryType::Text,
            values: memory,
        });

        result
    }

    fn set_memory(&mut self, mem_type: MemoryType, index: usize, value: u8) -> bool {
        match mem_type {
            MemoryType::Text => {
                if index >= self.processor.memory.memory.len() {
                    return false;
                }
                self.processor.memory.memory[index] = value;
                true
            }
            MemoryType::Data => false,
        }
    }

    fn get_registers(&mut self) -> Vec<RegisterState> {
        let mut result = Vec::new();

        result.push(RegisterState::new(
            "PC".to_string(),
            self.processor.registers.pc.into(),
        ));
        result.push(RegisterState::new(
            "FR".to_string(),
            self.processor.registers.fr.0.into(),
        ));
        result.push(RegisterState::new(
            "SP".to_string(),
            self.processor.registers.sp.into(),
        ));
        result.push(RegisterState::new(
            "BP".to_string(),
            self.processor.registers.bp.into(),
        ));
        result.push(RegisterState::new(
            "R00".to_string(),
            self.processor.registers.r[0].into(),
        ));
        result.push(RegisterState::new(
            "R01".to_string(),
            self.processor.registers.r[1].into(),
        ));
        result.push(RegisterState::new(
            "R02".to_string(),
            self.processor.registers.r[2].into(),
        ));
        result.push(RegisterState::new(
            "R03".to_string(),
            self.processor.registers.r[3].into(),
        ));

        result
    }

    fn load_executable(&mut self, binary: &[u8]) -> Result<(), String> {
        unimplemented!()
    }

    fn peek_stack(&mut self, n: u8) -> u16 {
        self.processor.peek_stack(n)
    }
}
