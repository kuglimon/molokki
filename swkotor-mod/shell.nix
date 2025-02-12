{
  pkgs ? import <nixpkgs> {},
  crane,
  fenix,
  system
}:
# FIXME(tatu): I don't know how to get rid of this duplication. Keep this file
# in sync with `package.nix`.
let
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
in
craneLib.devShell {
  # Take all inputs from the package, which gives us the full build environment
  buildInputs = [
    toolchain
    winCC
    pkgs.pkgsCross.mingw32.stdenv.cc
    pkgs.pkgsCross.mingw32.windows.pthreads
  ];

  shellHook = ''
    echo "Rust cross-compilation environment loaded!"
    export CARGO_BUILD_TARGET="i686-pc-windows-gnu";
    export TARGET_CC="${winCC}/bin/${winCC.targetPrefix}cc";
  '';
}
