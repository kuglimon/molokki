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
    buildInputs = with pkgs; [
      pipewire.dev
    ];
    nativeBuildInputs = with pkgs; [
      pkg-config
      rustPlatform.bindgenHook
    ];
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

    checkPhase = ''
      cargo test
    '';
  };

  devShell = (pkgs.mkShell.override { stdenv = pkgs.llvmPackages.stdenv; }) {
    buildInputs = with pkgs; [
      cargo
      alejandra
      pkg-config
      pipewire.dev
    ];

    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  };
}
