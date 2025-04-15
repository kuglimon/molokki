{
  pkgs,
}:
{
  package = pkgs.rustPlatform.buildRustPackage {
    pname = "fallout-save-editor";
    version = "0.1.1";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    checkPhase = ''
      cargo test
    '';
  };
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      cargo

      # Scripts under hacks use python
      (python3.withPackages (p: [ p.requests ]))
    ];
  };
}
