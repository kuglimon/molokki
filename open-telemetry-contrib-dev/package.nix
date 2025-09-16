{
  pkgs,
}:
let
  # I don't want to patch the build system included, hence FHS.
  fhs = pkgs.buildFHSEnv {
    name = "fhs-shell";
    targetPkgs = pkgs: [
      pkgs.gnumake
      pkgs.go
    ];
  };
in
{
  devShell = fhs.env;
}
