use std::process::Command;
use std::{env, fs};

pub fn exec_exists(binary: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, binary);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

pub fn install_exec(binary: &str) -> Result<(), String> {
    if exec_exists("pacman") {
        match Command::new("pacman")
            .arg("-S")
            .arg(" --noconfirm")
            .arg(binary)
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    } else if exec_exists("dnf") {
        match Command::new("dnf")
            .arg("install")
            .arg("-y")
            .arg(binary)
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    } else {
        Err("We currntly do not support your package manager".to_string())
    }
}
