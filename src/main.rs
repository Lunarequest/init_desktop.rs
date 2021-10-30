use shellexpand;
use std::fs::remove_file;
use std::path::Path;
mod clone;
mod utils;

fn main() {
    // check if ~/.dotfiles exists
    let dotfile_dir = shellexpand::tilde("~/.dotfiles").to_string();
    let path = Path::new(&dotfile_dir);
    if path.exists() {
        println!(
            "path {}, exists assuming dotfiles already cloned",
            dotfile_dir
        );
    } else {
        // if it doesn't exist git clone it
        let _dotfile_repo = clone::clone_repo(path, "https://github.com/Lunarequest/Dotfiles");
    }
    // check if stow is installed
    if !utils::exec_exists("stow") {
        // if it isn't, install it
        match utils::install_exec("stow") {
            Ok(_) => println!("installed stow"),
            Err(e) => panic!("{}", e),
        }
    }

    // install omz
    match utils::install_omz() {
        Ok(_) => println!("Installed omz"),
        Err(e) => println!("failed to install omz: {}", e),
    };
    
    // delete ~/.zshrc since we replace it
    remove_file(shellexpand::tilde("~/.zshrc"));

    // over here we stow all our dotfiles, GNU stow is awesome
    match utils::stow(path) {
        Ok(_) => println!("stowed dotfiles"),
        Err(e) => panic!("failed to stow dotfiles, exited with code {}", e),
    };
}
