use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn get_git_version() -> String {
    let version = env::var("CARGO_PKG_VERSION").unwrap().to_string();
    let child = Command::new("gitversion")
        .args(["/output", "json"])
        .output();
    match child {
        Ok(child) => String::from_utf8(child.stdout).expect("failed to read stdout"),
        Err(err) => {
            eprintln!("`gitversion` err: {}", err);
            version
        }
    }
}

fn main() {
    let version = get_git_version();
    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("GITVERSION")).unwrap();
    f.write_all(version.to_string().trim().as_bytes()).unwrap();
}
