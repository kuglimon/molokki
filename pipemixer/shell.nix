{
  pkgs ? import <nixpkgs> { },
}:
(pkgs.mkShell.override { stdenv = pkgs.llvmPackages.stdenv; }) {
  buildInputs = with pkgs; [
    cargo
    alejandra
    pkg-config
    pipewire.dev
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
