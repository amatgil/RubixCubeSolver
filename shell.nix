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
    gnuplot

    pkg-config
    xorg.libX11
    libGL
    alsa-lib
    xorg.libXi

    (rust-bin.stable.latest.default.override {
      targets = [ "wasm32-unknown-unknown" ];
    })

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
