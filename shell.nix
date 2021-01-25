{ pkgs ? import <nixpkgs> { overlays = [ (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)) ]; } }:

let
  rust = pkgs.latest.rustChannels.nightly.rust.override {
    extensions = [ "rust-src" ];
  };
in
pkgs.mkShell {
  buildInputs = [
    rust

    pkgs.bashInteractive
  ];
}
