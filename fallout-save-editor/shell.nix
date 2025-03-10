{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo

    # Scripts under hacks use python
    (python3.withPackages (p: [ p.requests ]))
  ];
}
