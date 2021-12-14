use core::panic;
use reqwest::blocking::get;
use std::env::{current_dir, set_current_dir};
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use tempfile::Builder;

pub fn exec_exists(binary: &str) -> bool {
    // first we get the PATH env var
    if let Ok(path) = env::var("PATH") {
        // then split it by : into a Vec<String>
        for p in path.split(':') {
            // format the string as p/binary so
            // if p = /usr/bin and binary = stow = /usr/bin/stow
            let p_str = format!("{}/{}", p, binary);
            // then we check if it exists
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

pub fn install_exec(binary: &str) -> Result<(), String> {
    // we check if a binary for pacman/dnf is in the path and use them to install the required deps
    // like stow
    if exec_exists("pacman") {
        match Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg(" --noconfirm")
            .arg(binary)
            .status()
        {
            Ok(status_code) => {
                if status_code.success() {
                    Ok(())
                } else {
                    Err(status_code.to_string())
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    } else if exec_exists("dnf") {
        match Command::new("sudo")
            .arg("dnf")
            .arg("install")
            .arg("-y")
            .arg(binary)
            .status()
        {
            Ok(status_code) => {
                if status_code.success() {
                    Ok(())
                } else {
                    Err(status_code.to_string())
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    } else if exec_exists("pkg") {
        match Command::new("pkg")
            .arg("install")
            .arg("-y")
            .arg(binary)
            .status()
        {
            Ok(status_code) => {
                if status_code.success() {
                    Ok(())
                } else {
                    Err(status_code.to_string())
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    } else if exec_exists("nix-env") {
        match Command::new("nix-env").arg("-iA").arg(binary).status() {
            Ok(status_code) => {
                if status_code.success() {
                    Ok(())
                } else {
                    Err(status_code.to_string())
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    } else {
        Err("We currntly do not support your package manager".to_string())
    }
}

pub fn install_omz() -> Result<(), String> {
    let target = "https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh";
    let response = get(target).unwrap();
    let tmp_dir = Builder::new().prefix("example").tempdir().unwrap();
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname).unwrap()
    };
    let content = response.text().unwrap();
    copy(&mut content.as_bytes(), &mut dest).unwrap();
    match Command::new("sh")
        .arg(format!("{}/install.sh", tmp_dir.path().display()))
        .status()
    {
        Ok(status_code) => {
            if status_code.success() {
                Ok(())
            } else {
                Err(status_code.to_string())
            }
        }
        Err(e) => panic!("{}", e),
    }
}

pub fn stow(path: &Path) -> Result<(), String> {
    // first we get a ReadDir of all the files/dirs in the path
    let blacklist: Vec<&str> = vec![
        "./.git",
        "./.github",
        "./nvim_old",
        "./.gitignore",
        "./.stylua.toml",
        "./README.md",
        "./wallpaper.png",
    ];
    let current_dir = current_dir().unwrap();
    assert!(set_current_dir(path).is_ok());
    let paths = fs::read_dir(".").unwrap();
    // then we generate a stow command and run it!
    let mut stow = Command::new("stow");
    for path in paths {
        let pathx = path.as_ref().unwrap().path();
        let x = pathx.as_os_str().to_str();
        let a = match x {
            Some(x) => x.to_string(),
            None => panic!(""),
        };
        //println!("{}:{}", a, blacklist.iter().any(|&i| i == a));
        if !path.as_ref().unwrap().path().is_file() && !blacklist.iter().any(|&i| i == a) {
            //println!("{}", path.as_ref().unwrap().path().display());
            stow.arg(
                path.unwrap()
                    .path()
                    .into_os_string()
                    .to_string_lossy()
                    .replace("./", ""),
            );
        }
    }
    //   println!("{:#?}", stow);
    match stow.current_dir(path).status() {
        Ok(status_code) => {
            // check if it returns a 0
            if status_code.success() {
                assert!(set_current_dir(current_dir).is_ok());
                Ok(())
            } else {
                Err(status_code.to_string())
            }
        }
        Err(e) => panic!("{}", e),
    }
}

pub fn cargo_install(package_name: &str) -> Result<(), String> {
    match Command::new("cargo")
        .arg("install")
        .arg(package_name)
        .status()
    {
        Ok(status_code) => {
            // check if it returns a 0
            if status_code.success() {
                Ok(())
            } else {
                Err(status_code.to_string())
            }
        }
        Err(e) => panic!("{}", e),
    }
}

pub fn install_deps() {
    if !exec_exists("zsh") {
        install_exec("zsh").expect("Zsh was not installed, failed to install zsh");
    }
    if !exec_exists("nvim") {
        install_exec("neovim").expect("Neovim was not installed, failed to install neovim");
    }
    if !exec_exists("vim") {
        install_exec("vim").expect("Vim was not installed, faileded to install vim");
    }
    if !exec_exists("curl") {
        install_exec("curl").expect("curl was not installed, failed to install curl");
    }
    if !exec_exists("ssh") {
        install_exec("ssh").expect("ssh was not installed, failed to install it")
    }
}

pub fn path_exists(path: String) -> bool {
    let path_as_path = Path::new(&path);
    return path_as_path.exists();
}
