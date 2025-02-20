use monistode_binutils::Architecture;
use monistode_binutils::Serializable;
use ux::u6;
use wasm_bindgen::prelude::*;

use crate::processor::WasmProcessor;
use crate::registers::RegisterState;
use crate::{MemoryBlock, MemoryType};
use monistode_binutils::Executable;
use monistode_emulator::common::Processor;
use monistode_emulator::stack_processor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct StackProcessorWrapper {
    processor: stack_processor::StackProcessor,
}

impl StackProcessorWrapper {
    pub fn new() -> Self {
        StackProcessorWrapper {
            processor: stack_processor::StackProcessor::new(),
        }
    }
}

impl WasmProcessor for StackProcessorWrapper {
    type ProcessorType = stack_processor::StackProcessor;
    type InstructionType = u6;

    fn get_processor(&self) -> &Self::ProcessorType {
        &self.processor
    }

    fn get_processor_mut(&mut self) -> &mut Self::ProcessorType {
        &mut self.processor
    }

    fn get_memory(&mut self) -> Vec<MemoryBlock> {
        let mut result = Vec::new();

        let mut text_memory = Vec::new();
        for value in self.processor.text_memory.memory.iter() {
            text_memory.push((*value).into());
        }
        result.push(MemoryBlock {
            memory_type: MemoryType::Text,
            values: text_memory,
        });

        let mut data_memory = Vec::new();
        for value in self.processor.data_memory.memory.iter() {
            data_memory.push(*value);
        }
        result.push(MemoryBlock {
            memory_type: MemoryType::Data,
            values: data_memory,
        });

        result
    }

    fn set_memory(&mut self, mem_type: MemoryType, index: usize, value: u8) -> bool {
        match mem_type {
            MemoryType::Text => {
                if value > u8::MAX - 1 {
                    return false;
                }
                self.processor.text_memory.memory[index] = u6::new(value);
                true
            }
            MemoryType::Data => {
                self.processor.data_memory.memory[index] = value;
                true
            }
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
            "TOS".to_string(),
            self.processor.registers.tos.into(),
        ));
        result.push(RegisterState::new(
            "SP".to_string(),
            self.processor.registers.sp.into(),
        ));

        result
    }

    fn load_executable(&mut self, binary: &[u8]) -> Result<(), String> {
        let executable = Executable::deserialize(binary)
            .map_err(|_| "Failed to load executable")?
            .1;
        if !matches!(executable.architecture(), Architecture::Stack) {
            return Err("Invalid architecture".to_string());
        }
        // log(&format!("Executable: {:?}", executable));
        self.processor.load_executable(&executable)
    }

    fn peek_stack(&mut self, n: u8) -> u16 {
        self.processor.peek_stack(n)
    }
}
