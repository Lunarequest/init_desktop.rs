{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	nativeBuildInputs = with pkgs; [ openssl openssl.dev ];
	PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
