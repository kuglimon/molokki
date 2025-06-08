{
  stdenv,
  pkgs,
}:
let
in
{

  package = stdenv.mkDerivation (finalAttrs: {
    pname = "load-balancing-ts";
    version = "v0.0.1";

    # TODO(tatu): Should never do this as, use filesets
    src = ./.;

    nativeBuildInputs = with pkgs; [
      nodejs
      pnpm.configHook
    ];

    pnpmDeps = pkgs.pnpm.fetchDeps {
      inherit (finalAttrs) pname version src;
      hash = "sha256-8/MU4RkQNl3B4UvDlRRU+lz3t/c8TTeo4zIpkgN0wmc=";
    };

    installPhase = ''
      runHook preInstall

      mkdir -p $out/{bin,lib/load-balancing-ts}
      cp -r {dist,node_modules} $out/lib/load-balancing-ts

      runHook postInstall
    '';

    buildPhase = ''
      runHook preBuild
      pnpm build
      runHook postBuild
    '';
  });

  devShell = pkgs.mkShell {
    buildInputs = [
      pkgs.pnpm
      pkgs.nodejs
    ];
  };
}
