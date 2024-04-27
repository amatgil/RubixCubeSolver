{
  pkgs ? import <nixpkgs> { },
  lib,
}:
let
  packages = with pkgs; [
    cargo
    rustc
    rust-analyzer
    rustfmt
    clippy
    clang
    mold

    #(rust-bin.stable.latest.default.override {
    # extensions = [ "rust-src" ];
    # targets = [ "wasm32-unknown-unknown" ];
    # })
  ];
in
pkgs.mkShell {
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
