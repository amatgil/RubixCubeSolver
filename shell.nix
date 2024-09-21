{
  pkgs ? import <nixpkgs> { },
  lib,
  overlays
}:
let
  packages = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
    mold
    cmake

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
  # Get dependencies from the main package
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}

