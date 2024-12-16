#![feature(rustc_private)]

extern crate rustc_codegen_cranelift;
extern crate rustc_codegen_ssa;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

use std::{collections::BTreeMap, path, process, str, sync::Arc};

// use rustc_codegen_llvm::LlvmCodegenBackend;
use rustc_errors::registry;
use rustc_hash::FxHashMap;
use rustc_session::config;
use rustc_interface::{Linker, passes};
use rustc_codegen_cranelift::{CraneliftCodegenBackend, BackendConfig, CodegenMode};
use rustc_codegen_ssa::traits::CodegenBackend;

fn main() {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    println!("sysroot: {}", sysroot);

    let working_dir = std::env::current_dir().expect("Current directory is invalid");
    let output_types = config::OutputTypes::new(&[(config::OutputType::Exe, None)]);
    // -Zcodegen-backend=cranelift

    let input = config::Input::Str {
        name: rustc_span::FileName::Custom("main.rs".into()),
        input: r#"
static HELLO: &str = "Hello, world!";
pub fn main() {
println!("{HELLO}");
}
"#
        .into(),
    };

    let backend = CraneliftCodegenBackend {
        config: Some(BackendConfig {
            codegen_mode: CodegenMode::Jit,
            jit_args: vec![],
        }),
    };
    backend.print_version();

    let mut config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            // maybe_sysroot: None,
            output_types,
            ..config::Options::default()
        },
        crate_cfg: Vec::new(),
        crate_check_cfg: Vec::new(),
        input: input,
        output_dir: None,  // Option<PathBuf>
        // output_file: Some(config::OutFileName::Real("./out.o".into())),
        output_file: None,
        ice_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES.to_vec(),
        lint_caps: Default::default(),
        psess_created: None,
        hash_untracked_state: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: Some(Box::new(|_opts| {
            // rustc_codegen_llvm::LlvmCodegenBackend::new()
            Box::new(backend)
        })),
        registry: registry::Registry::new(rustc_errors::codes::DIAGNOSTICS),
        using_internal_features: Arc::default(),
        expanded_args: Vec::new(),
    };


    // Inspired by https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_driver_impl/lib.rs.html
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|_queries| {
            let sess = &compiler.sess;

            let output_types = &sess.opts.output_types;
            println!("output_types: {:?}", output_types);

            let codegen_backend = &*compiler.codegen_backend;
            let linker = compiler.enter(|queries| {
                let early_exit = || {
                    sess.dcx().abort_if_errors();
                    None
                };

                // Parse the crate root source code (doesn't parse submodules yet)
                // Everything else is parsed during macro expansion.
                queries.parse();

                // Make sure name resolution and macro expansion is run.
                queries.global_ctxt().enter(|tcx| tcx.resolver_for_lowering());

                if false {
                    return early_exit();
                }

                queries.global_ctxt().enter(|tcx| {
                    passes::write_dep_info(tcx);
                    tcx.ensure().analysis(());
                    let output_filenames = tcx.output_filenames(());
                    println!("output_filenames: {:?}", output_filenames);
                    Some(Linker::codegen_and_build_linker(tcx, &*compiler.codegen_backend))
                })
            });

            // Linking is done outside the `compiler.enter()` so that the
            // `GlobalCtxt` within `Queries` can be freed as early as possible.
            if let Some(linker) = linker {
                linker.link(sess, codegen_backend);
            }
        });
    });
}
