use monistode_binutils::Architecture;
use monistode_binutils::Serializable;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::registers::RegisterState;
use crate::{MemoryBlock, MemoryType, WasmProcessor, WasmProcessorContinue};
use monistode_binutils::Executable;
use monistode_emulator::acc_processor;
use monistode_emulator::common::{Processor, ProcessorContinue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct AccProcessorWrapper {
    processor: acc_processor::AccProcessor,
}

impl AccProcessorWrapper {
    pub fn new() -> Self {
        AccProcessorWrapper {
            processor: acc_processor::AccProcessor::new(),
        }
    }
}

impl WasmProcessor for AccProcessorWrapper {
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
            "ACC".to_string(),
            self.processor.registers.acc.into(),
        ));
        result.push(RegisterState::new(
            "IR1".to_string(),
            self.processor.registers.ir1.into(),
        ));
        result.push(RegisterState::new(
            "IR2".to_string(),
            self.processor.registers.ir2.into(),
        ));

        result
    }

    fn load_executable(&mut self, binary: &[u8]) -> Result<(), String> {
        let executable = Executable::deserialize(binary)
            .map_err(|_| "Failed to load executable")?
            .1;
        if !matches!(executable.architecture(), Architecture::Accumulator) {
            return Err("Invalid architecture".to_string());
        }
        // log(&format!("Executable: {:?}", executable));
        self.processor.load_executable(&executable)
    }

    fn peek_stack(&mut self, n: u8) -> u16 {
        self.processor.peek_stack(n)
    }
}
