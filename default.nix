{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
rustPlatform.buildRustPackage rec {
  pname = "imposter-pass";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "129676ishpzlna3ggp56hz3vhcibwijxfxrbg7vd4mrnyczxw8cq";
  # verifyCargoDeps = true;
}
