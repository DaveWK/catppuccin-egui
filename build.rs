// SPDX-License-Identifier: MIT
use std::{
    fs,
    path::Path,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    println!("cargo::rerun-if-changed=src/themes.rs.tera");
    // Distributed crates omit the template; source builds can use the committed
    // palette without installing the optional maintainer generator.
    if !Path::new("src/themes.rs.tera").exists() {
        return ExitCode::SUCCESS;
    }
    let output = match Command::new("whiskers").arg("src/themes.rs.tera").output() {
        Ok(output) => output,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("could not run whiskers: {e}");
            return ExitCode::FAILURE;
        }
    };
    if !output.status.success() {
        eprintln!(
            "whiskers failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return ExitCode::FAILURE;
    }
    // Preserve the existing palette if generation fails; never truncate first.
    match fs::write("src/themes.rs", output.stdout) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("could not write generated palette: {e}");
            ExitCode::FAILURE
        }
    }
}
