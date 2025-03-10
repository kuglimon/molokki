{
  pkgs,
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "fallout-save-editor";
  version = "0.1.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  checkPhase = ''
    cargo test
  '';
}
