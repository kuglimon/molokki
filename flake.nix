{
  description = "Molokki is a monorepo containing variety of random software";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    # FIXME(tatu): This is a lie, I only test these on x86_64 linux and darwin.
    # I should setup proper systems.
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # FIXME(tatu): Move configuration back to subdirectories and use
        #              something like lib.filesystem.packagesFromDirectoryRecursive
        packages = {
          fallout-save-editor = pkgs.rustPlatform.buildRustPackage {
            pname = "fallout-save-editor";
            version = "0.1.1";
            cargoLock.lockFile = ./fallout-save-editor/Cargo.lock;
            src = pkgs.lib.cleanSource ./fallout-save-editor;

            checkPhase = ''
              cargo test
            '';
          };

          rojekti = let
            cargoToml = builtins.fromTOML (builtins.readFile ./rojekti/Cargo.toml);

            autosavevim = pkgs.writeShellScriptBin "autosavevim" ''
              # Used in integration tests to avoid process hanging on vim. Saves the
              # file and exists.
              echo "in path $PATH"
              ${pkgs.neovim}/bin/nvim -i NONE -u NONE -c ":wq" "$@"
            '';

            test-tmux = pkgs.writeShellScriptBin "tmux" ''
              # Used in integration tests to spawn a custom tmux-server. Users might
              # have their own servers running and we don't want to collide.
              echo "in path $PATH"
              ${pkgs.tmux}/bin/tmux -L rojekti-it "$@"
            '';
          in
          pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            nativeBuildInputs = with pkgs; [
              pkg-config
              rustPlatform.bindgenHook

              # Enable to debug failing builds
              # breakpointHook
            ];

            nativeCheckInputs = [
              autosavevim
              test-tmux
            ];

            cargoLock.lockFile = ./rojekti/Cargo.lock;
            src = pkgs.lib.cleanSource ./rojekti/.;

            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

            checkPhase = ''
              cargo test
            '';
          };
        };

        devShells = {
          fallout-save-editor = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo

              # Scripts under hacks use python
              (python3.withPackages (p: [ p.requests ]))
            ];
          };

          # FIXME(tatu): DevShell is broken atm as autosavevim and test-tmux are not
          # visible here.
          rojekti = pkgs.mkShell {
            buildInputs = with pkgs; [
              # FIXME(tatu): This is not optimal, this causes recompilation.
              # `runPackage` should be used instead?
              # FIXME(tatu): These are not working, not visible in cargo path
              #
              # Lord fucking mighty I love nix. Instead of fucking up users local
              # neovim with some random ass version required for tests, we can just
              # pass a special version to the command that needs it!
              (cargo.overrideAttrs (oldAttrs: {
                buildInputs = oldAttrs.buildInputs ++ [
                  autosavevim
                  test-tmux
                ];
              }))

              alejandra
              autosavevim
              test-tmux

              # tmuxinator for testing feature parity
              (ruby.withPackages (ps: with ps; [tmuxinator]))
            ];
          };
        };
      }
    );
}
