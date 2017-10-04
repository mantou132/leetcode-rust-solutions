// LICENSE of librustc_trans/back/lto.rs
// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private)]
extern crate rustc;
extern crate rustc_llvm;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_trans;
use rustc_llvm::*;

use std::ffi::CString;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;


fn main() {
    let args = &mut std::env::args();
    let options = &mut Vec::new();

    loop {
        let arg = args.next().unwrap();
        if arg == "--" {
            break
        }

        options.push(arg);
    }

    let matches = rustc_driver::handle_options(options.as_ref()).unwrap();
    let name = CString::new(args.next().unwrap()).unwrap();
    let (opts, _) = rustc::session::config::build_session_options_and_crate_config(&matches);

    let registry = rustc_driver::diagnostics_registry();
    let session = rustc::session::build_session(opts, None, registry);
    rustc_trans::init(&session);
    let tm = rustc_trans::back::write::create_target_machine(&session);

    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(name.as_ptr(), context);

        for filename in args {
            let file = &mut File::open(filename.as_str()).unwrap();
            let data = &mut Vec::new();
            let _ = file.read_to_end(data);
            if !LLVMRustLinkInExternalBitcode(module, data.as_ptr() as *const _, data.len()) {
                println!("failed to load {}", filename);
            }
        }

        let cstrs = vec!(CString::new("main").unwrap());
        let arr: Vec<*const _> = cstrs.iter().map(|c| c.as_ptr()).collect();
        LLVMRustRunRestrictionPass(module, arr.as_ptr(), arr.len());

        let pm = LLVMCreatePassManager();
        LLVMRustAddAnalysisPasses(tm, pm, module);
        let pass = LLVMRustFindAndCreatePass("verify\0".as_ptr() as *const _);
        assert!(!pass.is_null());
        LLVMRustAddPass(pm, pass);

        let builder = LLVMPassManagerBuilderCreate();

        LLVMPassManagerBuilderPopulateLTOPassManager(builder, pm, False, True);

        let pass = LLVMRustFindAndCreatePass("verify\0".as_ptr() as *const _);
        assert!(!pass.is_null());
        LLVMRustAddPass(pm, pass);
        LLVMRunPassManager(pm, module);

        LLVMRustWriteOutputFile(tm, pm, module, name.as_ptr(), FileType::AssemblyFile);
    }
}
