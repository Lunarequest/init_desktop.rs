{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	nativeBuildInputs = with pkgs; [ 
    openssl 
    openssl.dev 
    rust-analyzer
    binutils
    pkgconfig
    zsh
    any-nix-shell
    neovim
    ];
	PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
