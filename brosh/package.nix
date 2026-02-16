{
  pkgs,
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

  bashWithTests = import ./pkgs/bash { inherit pkgs; };

  run-all-tests = pkgs.writeShellApplication {
    name = "run-tests.sh";

    runtimeInputs = [
    ];

    text = builtins.readFile ./run-tests.sh;
  };
in
{
  package = pkgs.rustPlatform.buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;
    nativeBuildInputs = with pkgs; [
      pkg-config
      rustPlatform.bindgenHook

      # Enable to debug failing builds
      # breakpointHook
    ];

    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

    checkPhase = ''
      cargo test
    '';
  };

  devShell = pkgs.mkShell {
    buildInputs = [
    ];

    packages = [ run-all-tests ];
  };
}
