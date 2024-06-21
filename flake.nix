{
  description = "Default Rust flake (casenc)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs = (import nixpkgs { system = "x86_64-linux"; inherit overlays; });
      overlays = [ (import rust-overlay) ];
    in
    {
      devShells = forAllSystems (system: {
        default = pkgs.callPackage ./shell.nix { inherit pkgs overlays; };
      });
    };
}
