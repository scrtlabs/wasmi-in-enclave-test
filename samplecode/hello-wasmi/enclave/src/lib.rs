// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_trts;
extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

// extern crate parity_wasm;
extern crate wasmi;
// extern crate wabt;

// use parity_wasm::elements::{External, FunctionType, Internal, Type, ValueType};
use wasmi::{
    Error as InterpreterError, Externals, FuncInstance, FuncRef, HostError, ImportsBuilder,
    ModuleImportResolver, ModuleInstance, ModuleRef, RuntimeArgs, RuntimeValue, Signature, Trap,
    ValueType,
};

use sgx_trts::enclave;
use sgx_types::metadata::*;
use sgx_types::*;
//use sgx_trts::{is_x86_feature_detected, is_cpu_feature_supported};
use std::backtrace::{self, PrintFormat};
use std::io::{self, Write};
use std::slice;
use std::string::String;
use std::vec::Vec;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {
    // Ocall to normal world for output
    // println!("{}", &hello_string);

    let _ = backtrace::enable_backtrace("enclave.signed.so", PrintFormat::Full);

    let slice = unsafe { slice::from_raw_parts(some_string, some_len) };

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&slice).expect("failed to load wasm");

    let imports = ImportsBuilder::new().with_resolver("get_the_number_two", &ResolveAll);
    let mut runtime = Runtime {};

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance = ModuleInstance::new(&module, &imports)
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    assert_eq!(
        instance
            .invoke_export("test", &[], &mut runtime)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1339)),
    );

    sgx_status_t::SGX_SUCCESS
}

struct Runtime;

const GET_TWO_INDEX: usize = 0;

impl Externals for Runtime {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            GET_TWO_INDEX => Ok(Some(RuntimeValue::I32(2))),
            _ => panic!("unknown function index"),
        }
    }
}

struct ResolveAll;

impl ModuleImportResolver for ResolveAll {
    fn resolve_func(&self, _field_name: &str, signature: &Signature) -> Result<FuncRef, InterpreterError> {
        let func_ref = match _field_name {
            "__get_the_number_two" => FuncInstance::alloc_host(
                Signature::new(&[][..], Some(ValueType::I32)),
                GET_TWO_INDEX,
            ),
            _ => {
                return Err(InterpreterError::Function(format!(
                    "host module doesn't export function with name {}",
                    _field_name
                )));
            }
        };
        Ok(func_ref)
    }
}
