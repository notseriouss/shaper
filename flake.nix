{
  description = "Shaper - template manager";

  inputs = {
    nixpkgs = {
      type  = "github";
      owner = "NixOS";
      repo  = "nixpkgs";
      ref   = "nixos-unstable";
      flake = true;
    };

    flake-utils = {
      type  = "github";
      owner = "numtide";
      repo  = "flake-utils";
      flake = true;
    };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        package = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "shaper";
          version = package.version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          meta = {
            description = package.description;
            homepage = "https://github.com/notseriouss/shaper";
            license = pkgs.lib.licenses.gpl3Only;
            mainProgram = "shaper";
          };
        };
      }
    );
}
