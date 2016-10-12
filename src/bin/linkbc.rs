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
use std::rc::Rc;


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

    let dep_graph = rustc::dep_graph::DepGraph::new(false);
    let registry = rustc_driver::diagnostics_registry();
    let cstore = Rc::new(rustc_metadata::cstore::CStore::new(&dep_graph));
    let session = rustc::session::build_session(opts, &dep_graph, None, registry, cstore.clone());
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
