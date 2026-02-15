{
  pkgs,
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
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
  };
}
