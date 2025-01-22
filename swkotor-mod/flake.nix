{
  description = "rojekti flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, crane, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # Stolen from: https://github.com/deltachat/deltachat-core-rust/blob/8dcd8aa69d600ab5847bd1c38a08aee38af7c844/flake.nix#L159
        # Get rid of MCF Gthread library.
        # See <https://github.com/NixOS/nixpkgs/issues/156343>
        # and <https://discourse.nixos.org/t/statically-linked-mingw-binaries/38395>
        # for details.
        #
        # Use DWARF-2 instead of SJLJ for exception handling.
        winCC = pkgs.pkgsCross.mingw32.buildPackages.wrapCC (
          (pkgs.pkgsCross.mingw32.buildPackages.gcc-unwrapped.override
            ({
              threadsCross = {
                model = "win32";
                package = null;
              };
            })).overrideAttrs (oldAttr: {
            configureFlags = oldAttr.configureFlags ++ [
              "--disable-sjlj-exceptions --with-dwarf2"
            ];
          })
        );

        toolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            targets.i686-pc-windows-gnu.latest.rust-std
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        my-crate = craneLib.buildPackage rec {
          src = craneLib.cleanCargoSource ./.;

          strictDeps = true;
          doCheck = false;

          CARGO_BUILD_TARGET = "i686-pc-windows-gnu";

          TARGET_CC   = "${winCC}/bin/${winCC.targetPrefix}cc";

          CARGO_BUILD_RUSTFLAGS = [
            "-C"
            "linker=${TARGET_CC}"
          ];

          #fixes issues related to openssl
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

          CC = "${winCC}/bin/${winCC.targetPrefix}cc";
          LD = "${winCC}/bin/${winCC.targetPrefix}cc";

          depsBuildBuild = with pkgs; [
            pkgsCross.mingw32.stdenv.cc
            pkgsCross.mingw32.windows.pthreads
          ];
        };
      in
      {
        packages = {
          inherit my-crate;
          default = my-crate;
        };

        checks = {
          inherit my-crate;
        };
      }
    );
}
