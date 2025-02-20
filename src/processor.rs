use std::future::Future;

use crate::{
    memory::{MemoryBlock, MemoryType},
    processors::{acc::AccProcessorWrapper, cisc::CiscProcessorWrapper, risc::RiscProcessorWrapper, stack::StackProcessorWrapper},
    registers::RegisterState,
};
use js_sys::Promise;
use monistode_emulator::common::{Processor, ProcessorContinue};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum WasmProcessorContinue {
    Continue,
    Error,
    Halt,
}

pub trait WasmProcessor {
    type ProcessorType: Processor<Self::InstructionType, u16, u16, u16>;
    type InstructionType: Copy + Into<u8>;

    fn get_processor(&self) -> &Self::ProcessorType;
    fn get_processor_mut(&mut self) -> &mut Self::ProcessorType;

    fn get_memory(&mut self) -> Vec<MemoryBlock>;
    fn set_memory(&mut self, mem_type: MemoryType, index: usize, value: u8) -> bool;
    fn get_registers(&mut self) -> Vec<RegisterState>;
    fn load_executable(&mut self, binary: &[u8]) -> Result<(), String>;
    fn peek_stack(&mut self, n: u8) -> u16;

    fn run(
        &'static mut self,
        output: &'static js_sys::Function,
        input: &'static js_sys::Function,
    ) -> Box<dyn Future<Output = WasmProcessorContinue>> {
        Box::new(async move {
            let result = self
                .get_processor_mut()
                .run_command(
                    move |port, value| {
                        Box::new(async move {
                            let _ = output.call2(
                                &JsValue::NULL,
                                &JsValue::from_f64(port as f64),
                                &JsValue::from_f64(value as f64),
                            );
                        })
                    },
                    |port| {
                        Box::new(async move {
                            let promise: Promise = input
                                .call1(&JsValue::NULL, &JsValue::from_f64(port as f64))
                                .unwrap()
                                .dyn_into()
                                .unwrap();
                            let value =
                                wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                            value.as_f64().unwrap() as u16
                        })
                    },
                )
                .await;
            match result {
                ProcessorContinue::KeepRunning => WasmProcessorContinue::Continue,
                ProcessorContinue::Error => WasmProcessorContinue::Error,
                ProcessorContinue::Halt => WasmProcessorContinue::Halt,
            }
        })
    }
}

pub enum WasmProcessorEnum {
    Stack(StackProcessorWrapper),
    Acc(AccProcessorWrapper),
    Risc(RiscProcessorWrapper),
    Cisc(CiscProcessorWrapper),
}

impl WasmProcessorEnum {
    pub fn get_memory(&mut self) -> Vec<MemoryBlock> {
        match self {
            WasmProcessorEnum::Stack(p) => p.get_memory(),
            WasmProcessorEnum::Acc(p) => p.get_memory(),
            WasmProcessorEnum::Risc(p) => p.get_memory(),
            WasmProcessorEnum::Cisc(p) => p.get_memory(),
        }
    }

    pub fn set_memory(&mut self, mem_type: MemoryType, index: usize, value: u8) -> bool {
        match self {
            WasmProcessorEnum::Stack(p) => p.set_memory(mem_type, index, value),
            WasmProcessorEnum::Acc(p) => p.set_memory(mem_type, index, value),
            WasmProcessorEnum::Risc(p) => p.set_memory(mem_type, index, value),
            WasmProcessorEnum::Cisc(p) => p.set_memory(mem_type, index, value),
        }
    }

    pub fn get_registers(&mut self) -> Vec<RegisterState> {
        match self {
            WasmProcessorEnum::Stack(p) => p.get_registers(),
            WasmProcessorEnum::Acc(p) => p.get_registers(),
            WasmProcessorEnum::Risc(p) => p.get_registers(),
            WasmProcessorEnum::Cisc(p) => p.get_registers(),
        }
    }

    pub fn load_executable(&mut self, binary: &[u8]) -> Result<(), String> {
        match self {
            WasmProcessorEnum::Stack(p) => p.load_executable(binary),
            WasmProcessorEnum::Acc(p) => p.load_executable(binary),
            WasmProcessorEnum::Risc(p) => p.load_executable(binary),
            WasmProcessorEnum::Cisc(p) => p.load_executable(binary),
        }
    }

    pub fn peek_stack(&mut self, n: u8) -> u16 {
        match self {
            WasmProcessorEnum::Stack(p) => p.peek_stack(n),
            WasmProcessorEnum::Acc(p) => p.peek_stack(n),
            WasmProcessorEnum::Risc(p) => p.peek_stack(n),
            WasmProcessorEnum::Cisc(p) => p.peek_stack(n),
        }
    }

    pub fn run(
        &'static mut self,
        output: &'static js_sys::Function,
        input: &'static js_sys::Function,
    ) -> Box<dyn Future<Output = WasmProcessorContinue>> {
        match self {
            WasmProcessorEnum::Stack(p) => p.run(output, input),
            WasmProcessorEnum::Acc(p) => p.run(output, input),
            WasmProcessorEnum::Risc(p) => p.run(output, input),
            WasmProcessorEnum::Cisc(p) => p.run(output, input),
        }
    }
}
