use shellexpand;
use std::path::Path;
mod clone;
mod installdep;

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
    if !installdep::exec_exists("stow") {
        // if it isn't, install it
        match installdep::install_exec("stow") {
            Ok(()) => println!("installed stow"),
            Err(e) => panic!("{}", e),
        }
    }
}
