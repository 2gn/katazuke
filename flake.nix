{
  description = "A very basic flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      rec {
        packages = flake-utils.lib.flattenTree {
          katazuke = stdenv.mkDerivation {
            name = "katazuke";
            src = ./.;
            buildInputs = [
              rust-bin.beta.latest.default
            ] ++ lib.optionals stdenv.isDarwin [
              libiconv
            ];
          };
        };
        defaultPackage = packages.katazuke;
      }
    );
}
