#![feature(prelude_import)]
#![feature(libstd_sys_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(rt)]
#![feature(print_internals)]
#![feature(derive_eq)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(coverage_attribute)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::ffi::c_void;
use std::boxed::Box;
use com::sys::IID;
mod provider;
use provider::RustAmsiProvider;
use com::interfaces;

const CLSID_IID: IID = IID {
    data1: 0xf3d06e7c,
    data2: 0x1e45,
    data3: 0x4a26,
    data4: [0x84, 0x7e, 0xf9, 0xfc, 0xde, 0xe5, 0x9b, 0xe1],
};

// 0xb2cabfe3_fe04_42b1_a5df_08d483d4d125
const IAntimalwareProvider_IID: IID = IID {
    data1: 0xb2cabfe3,
    data2: 0xfe04,
    data3: 0x42b1,
    data4: [0xa5, 0xdf, 0x08, 0xd4, 0x83, 0xd4, 0xd1, 0x25],
};

com::inproc_dll_module![(CLSID_IID, RustAmsiProvider),];
