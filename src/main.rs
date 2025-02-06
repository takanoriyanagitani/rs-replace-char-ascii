use std::process::ExitCode;

use std::io;

use rs_replace_char_ascii::replace::cfg::ReplaceConfig;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key)
        .map_err(|e| io::Error::other(format!("unable to get an environment variable {key}: {e}")))
}

fn string2u8(i: String) -> Result<u8, io::Error> {
    let s: &[u8] = i.as_bytes();
    s.first()
        .copied()
        .ok_or_else(|| io::Error::other("empty string got"))
}

fn replace_char_from_env(key: &'static str) -> Result<u8, io::Error> {
    env_val_by_key(key).and_then(string2u8)
}

fn before() -> Result<u8, io::Error> {
    replace_char_from_env("ENV_REPLACE_BEFORE")
}

fn after() -> Result<u8, io::Error> {
    replace_char_from_env("ENV_REPLACE_AFTER")
}

fn replace_config() -> Result<ReplaceConfig, io::Error> {
    let b: u8 = before()?;
    let a: u8 = after()?;
    Ok(ReplaceConfig {
        before: b,
        after: a,
    })
}

fn stdin2replaced2stdout_std() -> Result<(), io::Error> {
    let cfg: ReplaceConfig = replace_config()?;
    rs_replace_char_ascii::replace::std::stdin2replaced2stdout(cfg.before, cfg.after)
}

#[cfg(not(all(target_os = "wasi", feature = "enable_simd")))]
fn stdin2replaced2stdout_wasi_simd() -> Result<(), io::Error> {
    Ok(()) // nop
}

#[cfg(all(target_os = "wasi", feature = "enable_simd"))]
fn stdin2replaced2stdout_wasi_simd() -> Result<(), io::Error> {
    let cfg: ReplaceConfig = replace_config()?;
    rs_replace_char_ascii::replace::wasm::simd::stdin2replaced2stdout(cfg.before, cfg.after)
}

fn stdin2replaced2stdout() -> Result<(), io::Error> {
    if cfg!(not(all(target_os = "wasi", feature = "enable_simd"))) {
        stdin2replaced2stdout_std()?;
    }

    if cfg!(all(target_os = "wasi", feature = "enable_simd")) {
        stdin2replaced2stdout_wasi_simd()?;
    }

    Ok(())
}

fn main() -> ExitCode {
    stdin2replaced2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
