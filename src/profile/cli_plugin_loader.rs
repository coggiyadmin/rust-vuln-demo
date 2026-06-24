// FP-target (upstream cognium-dev#162/#140) — DEEP reflective-load flow. A dev/CLI tool
// dynamically loads a shared library by path and resolves+calls a symbol via libloading. The
// path/symbol are operator-controlled tool arguments, not an HTTP source, so this must NOT be
// flagged code_injection under an entry-point gate — even though a real dynamic-load +
// dynamic-dispatch sink is present.
#![allow(dead_code)]
use libloading::{Library, Symbol};

/// Loads a plugin selected by the operator on the command line and calls its `run` symbol.
pub unsafe fn load_and_run(so_path: &str, symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lib = Library::new(so_path)?; // operator-controlled path, not request data
    let run: Symbol<unsafe extern "C" fn()> = lib.get(symbol.as_bytes())?; // dynamic symbol resolution
    run();
    Ok(())
}
