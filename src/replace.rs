pub mod cfg;

pub mod std;

#[cfg(target_os = "wasi")]
pub mod wasm;
