{
  description = "rojekti flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
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

      checkPhase = ''
        cargo test
      '';
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [];
    };
  };
}
