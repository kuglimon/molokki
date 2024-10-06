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

      nativeCheckInputs = with pkgs; [
        neovim
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
        # Lord fucking mighty I love nix. Instead of fucking up users local
        # neovim with some random ass version required for tests, we can just
        # pass a special version to the command that needs it!
        (cargo.overrideAttrs (oldAttrs: {
          buildInputs = oldAttrs.buildInputs ++ [ neovim ];
        }))

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
