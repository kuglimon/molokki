{
  description = "pipemixer flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
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
    };

    devShells.${system}.default = (pkgs.mkShell.override {stdenv = pkgs.llvmPackages.stdenv;}) {
      buildInputs = with pkgs; [
        cargo
        alejandra
        pkg-config
        pipewire.dev
      ];

      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
    };

    checks.x86_64-linux.alejandro = with nixpkgs.legacyPackages.x86_64-linux;
      runCommand "format" {
        nativeBuildInputs = with pkgs; [alejandra];
      } ''
        alejandra -c ${self}
        touch $out
      '';
  };
}
