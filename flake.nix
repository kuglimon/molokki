{
  description = "Molokki is a monorepo containing variety of random software";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      fenix,
    }:
    # FIXME(tatu): This is a lie, I only test these on x86_64 linux and darwin.
    # I should setup proper systems.
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
      in
      {
        # FIXME(tatu): Move configuration back to subdirectories and use
        #              something like lib.filesystem.packagesFromDirectoryRecursive
        # XXX(tatu): I wonder if it would be easier to move all projects under
        #            something like pkgs?
        packages = rec {
          fallout-save-editor = pkgs.callPackage ./fallout-save-editor/package.nix { };
          rojekti = pkgs.callPackage ./rojekti/package.nix { };
          pipemixer = pkgs.callPackage ./pipemixer/package.nix { };
          poe-trade-overlay = pkgs.callPackage ./poe-trade-overlay/package.nix { };
          swkotor-mod = pkgs.callPackage ./swkotor-mod/package.nix {
            inherit crane;
            inherit fenix;
          };

          default = rojekti;

          # This output is used to generate the job matrix in github to build
          # each output separately.
          build-matrix = pkgs.writeText "projects.json" (
            builtins.toJSON {
              # Get all our projects while filtering out the default, otherwise
              # we'd build it twice.
              project = (lib.lists.remove "default" (builtins.attrNames self.outputs.packages.x86_64-linux));
              # FIXME(tatu): This should support other architectures like darwin
              # and arm
              os = [ "ubuntu-latest" ];
            }
          );
        };

        formatter = pkgs.nixfmt-rfc-style;

        # shell.nix is mostly just for extra packages. If you just need the
        # packages you need for building 'nix develop' and 'nix-shell' are
        # smart enough to use 'package.nix'.
        devShells = {
          fallout-save-editor = import ./fallout-save-editor/shell.nix {
            inherit pkgs;
          };
          rojekti = import ./rojekti/shell.nix { inherit pkgs; };
          pipemixer = import ./pipemixer/shell.nix { inherit pkgs; };
          poe-trade-overlay = import ./poe-trade-overlay/shell.nix {
            inherit pkgs;
          };
          swkotor-mod = import ./swkotor-mod/shell.nix {
            inherit pkgs;
            inherit crane;
            inherit fenix;
            inherit system;
          };
        };
      }
    );
}
