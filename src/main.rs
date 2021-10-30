use shellexpand;
use std::path::Path;
mod clone;
mod installdep;

fn main() {
    let dotfile_dir = shellexpand::tilde("~/.dotfiles").to_string();
    let path = Path::new(&dotfile_dir);
    if !path.exists() {
        println!(
            "path {}, exists assuming dotfiles already cloned",
            dotfile_dir
        );
    } else {
        let _dotfile_repo = clone::clone_repo(path, "https://github.com/Lunarequest/Dotfiles");
    }
    if installdep::exec_exists("stow") {
        match installdep::install_exec("stow") {
            Ok(()) => println!("installed stow"),
            Err(e) => panic!("{}", e),
        }
    }
}
