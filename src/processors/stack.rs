use ux::u6;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::registers::RegisterState;
use crate::{MemoryBlock, MemoryType, WasmProcessor, WasmProcessorContinue};
use monistode_emulator::common::{Processor, ProcessorContinue};
use monistode_emulator::executable::Executable;
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
    fn run(
        &mut self,
        output: &js_sys::Function,
        input: &js_sys::Function,
    ) -> WasmProcessorContinue {
        let result = self.processor.run_command(
            |port, value| {
                let _ = output.call2(
                    &JsValue::NULL,
                    &JsValue::from_f64(port as f64),
                    &JsValue::from_f64(value as f64),
                );
            },
            |port| {
                let value = input.call1(&JsValue::NULL, &JsValue::from_f64(port as f64));
                if let Ok(value) = value {
                    value.as_f64().unwrap() as u16
                } else {
                    0
                }
            },
        );
        match result {
            ProcessorContinue::KeepRunning => WasmProcessorContinue::Continue,
            ProcessorContinue::Error => WasmProcessorContinue::Error,
            ProcessorContinue::Halt => WasmProcessorContinue::Halt,
        }
    }

    fn run_n(
        &mut self,
        output: &js_sys::Function,
        input: &js_sys::Function,
        n: usize,
    ) -> WasmProcessorContinue {
        for _ in 0..n {
            let result = self.processor.run_command(
                |port, value| {
                    let _ = output.call2(
                        &JsValue::NULL,
                        &JsValue::from_f64(port as f64),
                        &JsValue::from_f64(value as f64),
                    );
                },
                |port| {
                    let value = input.call1(&JsValue::NULL, &JsValue::from_f64(port as f64));
                    if let Ok(value) = value {
                        value.as_f64().unwrap() as u16
                    } else {
                        0
                    }
                },
            );
            match result {
                ProcessorContinue::KeepRunning => {}
                ProcessorContinue::Error => return WasmProcessorContinue::Error,
                ProcessorContinue::Halt => return WasmProcessorContinue::Halt,
            }
        }
        WasmProcessorContinue::Continue
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
        let executable = Executable::new(binary);
        // log(&format!("Executable: {:?}", executable));
        self.processor.load_executable(&executable)
    }
}
