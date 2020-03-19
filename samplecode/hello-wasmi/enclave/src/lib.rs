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

extern crate sgx_types;
extern crate sgx_trts;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate parity_wasm;
extern crate wasmi;

use parity_wasm::elements::{External, FunctionType, Internal, Type, ValueType};
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

use sgx_types::*;
use sgx_types::metadata::*;
use sgx_trts::enclave;
//use sgx_trts::{is_x86_feature_detected, is_cpu_feature_supported};
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::backtrace::{self, PrintFormat};

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    // Ocall to normal world for output
    println!("{}", &hello_string);

    let _ = backtrace::enable_backtrace("enclave.signed.so", PrintFormat::Full);

    sgx_status_t::SGX_SUCCESS
}