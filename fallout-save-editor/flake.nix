{
  description = "fallout-save-editor flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "fallout-save-editor";
      version = "0.1.0";
      cargoLock.lockFile = ./Cargo.lock;
      src = pkgs.lib.cleanSource ./.;
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo

        # tmuxinator for testing feature parity
        (ruby.withPackages (ps: with ps; [ tmuxinator ]))

        # Scripts under hacks use python
        (python3.withPackages (p: [ p.requests ]))
      ];
    };
  };
}
