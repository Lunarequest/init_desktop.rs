use std::fs::remove_file;
use std::path::Path;
mod clone;
mod utils;

fn main() {
    // check if ~/.dotfiles exists
    match utils::install_deps() {
        Ok(()) => println!("installed deps"),
        Err(e) => panic!("{}", e),
    }
    let dotfile_dir = shellexpand::tilde("~/.dotfiles").to_string();
    let omz_dir = shellexpand::tilde("~/.oh-my-zsh").to_string();
    let omz_path = Path::new(&omz_dir);
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
    if omz_path.exists() {
        println!("path {} exists assuming omz is already installed", omz_dir)
    } else {
        match utils::install_omz() {
            Ok(_) => println!("Installed omz"),
            Err(e) => println!("failed to install omz: {}", e),
        };

        // delete ~/.zshrc since we replace it
        match remove_file(&*shellexpand::tilde("~/.zshrc")) {
            Ok(_) => println!("removed omz provided zshrc"),
            Err(e) => panic!("Failed to remove omz provided zshrc: {}", e),
        };
    }

    // over here we stow all our dotfiles, GNU stow is awesome
    match utils::stow(path) {
        Ok(_) => println!("stowed dotfiles"),
        Err(e) => panic!("failed to stow dotfiles, exited with code {}", e),
    };
    // now we install some deps of the zshrc
    println!("installing zsh-autosuggestions");
    clone::clone_repo(
        Path::new(&*shellexpand::tilde(
            "~/.oh-my-zsh/custom/plugins/zsh-autosuggestions",
        )),
        "https://github.com/zsh-users/zsh-autosuggestions.git",
    );
    println!("installing zsh histroy substring search");
    clone::clone_repo(
        Path::new(&*shellexpand::tilde(
            "~/.oh-my-zsh/custom/plugins/zsh-history-substring-search",
        )),
        "https://github.com/zsh-users/zsh-history-substring-search.git",
    );
    println!("installing zsh syntax highlighting");
    clone::clone_repo(
        Path::new(&*shellexpand::tilde(
            "~/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting",
        )),
        "https://github.com/zsh-users/zsh-syntax-highlighting.git",
    );
    println!("installing powerlevel10k");
    clone::clone_repo(
        Path::new(&*shellexpand::tilde(
            "~/.oh-my-zsh/custom/themes/powerlevel10k",
        )),
        "https://github.com/romkatv/powerlevel10k.git",
    );
    println!("installing vim packager");
    clone::clone_repo(
        Path::new(&*shellexpand::tilde(
            "~/.vim/pack/packager/opt/vim-packager",
        )),
        "https://github.com/kristijanhusak/vim-packager.git",
    );
    match utils::cargo_install("vivid") {
        Ok(_) => println!("installed vivid"),
        Err(e) => panic!("{}", e),
    };
}
