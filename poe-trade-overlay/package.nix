{
  pkgs
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "path-of-exile-price-overlay";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
