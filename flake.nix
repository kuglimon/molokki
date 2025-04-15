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
    } @ inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" "x86_64-darwin" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
        callPackage = pkgs.lib.callPackageWith (pkgs // inputs);
        root = ./.;

        # Find all nix based projects, not all have been converted
        nixProjects = lib.filterAttrs (name: type:
          type == "directory" &&
          builtins.pathExists (root + "/${name}/package.nix")
        ) (builtins.readDir root);

        # Import all projects and separate derivation and attrset based ones.
        # Derivations are the old way I used to use where package.nix and
        # shell.nix were separate files.
        #
        # Using separate files lead to issues where it was harder to share
        # configuration with package.nix and shell.nix. To be fair, most of that
        # was just me being fucking stupid. But it was annoying to flick around
        # the files.
        #
        # Remember that there's not really much magic in nix, it's just a
        # programming language with some agreed shape of attributes. Learn the
        # difference between attribute sets, derivations and functions, you're
        # 95% done after that.
        #
        # FIXME(tatu): evaluated, derivations and attrsets could be done in one
        # statement, this is just overly complex.
        evaluated = lib.mapAttrs (name: _:
            # Note that this requires a callPackage that has inputs as default
            # attrs, as we have packages depending on inputs like fenix and
            # crane
            callPackage (root + "/${name}/package.nix") { }
        ) nixProjects;

        # Split into derivations and non-derivation attrsets
        derivations = lib.filterAttrs (_: v: lib.isDerivation v) evaluated;
        attrsets = lib.filterAttrs (_: v: builtins.isAttrs v && !lib.isDerivation v) evaluated;
      in
      {
        # FIXME(tatu): Move configuration back to subdirectories and use
        #              something like lib.filesystem.packagesFromDirectoryRecursive
        # XXX(tatu): I wonder if it would be easier to move all projects under
        #            something like pkgs?
        packages = {
          # XXX(tatu): Should the default just build everything?
          default = self.packages.${system}.rojekti;

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
        } // derivations // lib.mapAttrs (name: value: value.package) attrsets;

        formatter = pkgs.nixfmt-rfc-style;

        # shell.nix is mostly just for extra packages. If you just need the
        # packages you need for building 'nix develop' and 'nix-shell' are
        # smart enough to use 'package.nix'.
        devShells = {
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
        } // lib.mapAttrs (name: value: value.devShell) attrsets;
      }
    );
}
