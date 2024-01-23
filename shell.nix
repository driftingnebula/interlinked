{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  rustup-toolchain = rust-bin.fromRustupToolchainFile ./rustup-toolchain.toml;
in
mkShell rec {
  packages = [
    cargo-audit
    cargo-edit
    cargo-expand
    cargo-insta
    cargo-make
    cargo-outdated
    gegl.dev
    imagemagick
    rustup-toolchain
  ];
}
