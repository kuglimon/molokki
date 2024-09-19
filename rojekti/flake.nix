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

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        alejandra

        # tmuxinator for testing feature parity
        (ruby.withPackages (ps: with ps; [tmuxinator]))
      ];
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
