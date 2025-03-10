// SPDX-License-Identifier: Apache-2.0

use path_slash::PathExt;
use solang::{codegen, file_resolver::FileResolver, parse_and_resolve, Target};
use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

#[test]
fn solana_contracts() -> io::Result<()> {
    contract_tests("tests/contract_testcases/solana", Target::Solana)
}

#[test]
fn substrate_contracts() -> io::Result<()> {
    contract_tests(
        "tests/contract_testcases/substrate",
        Target::default_substrate(),
    )
}

#[test]
fn ewasm_contracts() -> io::Result<()> {
    contract_tests("tests/contract_testcases/ewasm", Target::Ewasm)
}

fn contract_tests(file_path: &str, target: Target) -> io::Result<()> {
    let path = PathBuf::from(file_path);
    recurse_directory(path, target)
}

fn recurse_directory(path: PathBuf, target: Target) -> io::Result<()> {
    for entry in read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            recurse_directory(path, target)?;
        } else if let Some(ext) = path.extension() {
            if ext.to_string_lossy() == "sol" {
                parse_file(path, target)?;
            }
        }
    }

    Ok(())
}

fn parse_file(path: PathBuf, target: Target) -> io::Result<()> {
    let mut cache = FileResolver::new();

    let filename = add_file(&mut cache, &path, target)?;

    let mut ns = parse_and_resolve(OsStr::new(&filename), &mut cache, target);

    if !ns.diagnostics.any_errors() {
        // codegen all the contracts
        codegen::codegen(
            &mut ns,
            &codegen::Options {
                math_overflow_check: false,
                opt_level: codegen::OptimizationLevel::Default,
                ..Default::default()
            },
        );
    }

    #[cfg(windows)]
    {
        for file in &mut ns.files {
            let filename = file.path.to_slash_lossy().to_string();
            file.path = PathBuf::from(filename);
        }
    }

    if !ns.diagnostics.any_errors() {
        let context = inkwell::context::Context::create();

        // let's try and emit
        if ns.target == Target::Solana {
            solang::emit::binary::Binary::build_bundle(
                &context,
                &[&ns],
                &filename,
                Default::default(),
                false,
                false,
            );
        } else {
            for contract in &ns.contracts {
                if contract.is_concrete() {
                    solang::emit::binary::Binary::build(
                        &context,
                        contract,
                        &ns,
                        &filename,
                        Default::default(),
                        false,
                        false,
                    );
                }
            }
        }
    }

    let mut path = path;

    path.set_extension("dot");

    let generated_dot = ns.dotgraphviz();

    // uncomment the next three lines to regenerate the test data
    // use std::io::Write;
    // let mut file = File::create(&path)?;
    // file.write_all(generated_dot.as_bytes())?;

    let mut file = File::open(&path)?;

    let mut test_dot = String::new();

    file.read_to_string(&mut test_dot)?;

    // The dot files may have had their end of lines mangled on Windows
    let test_dot = test_dot.replace("\r\n", "\n");

    pretty_assertions::assert_eq!(generated_dot, test_dot);

    Ok(())
}

fn add_file(cache: &mut FileResolver, path: &Path, target: Target) -> io::Result<String> {
    let mut file = File::open(&path)?;

    let mut source = String::new();

    file.read_to_string(&mut source)?;

    // make sure the path uses unix file separators, this is what the dot file uses
    let filename = path.to_slash_lossy();

    println!("Parsing {} for {}", filename, target);

    // The files may have had their end of lines mangled on Windows
    cache.set_file_contents(&filename, source.replace("\r\n", "\n"));

    for line in source.lines() {
        if line.starts_with("import") {
            let start = line.find('"').unwrap();
            let end = line.rfind('"').unwrap();
            let file = &line[start + 1..end];
            if file != "solana" {
                let mut import_path = path.parent().unwrap().to_path_buf();
                import_path.push(file);
                println!("adding import {}", import_path.display());
                add_file(cache, &import_path, target)?;
            }
        }
    }

    Ok(filename.to_string())
}
