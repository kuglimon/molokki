{
  stdenv,
  pkgs,
  pnpmConfigHook,
  fetchPnpmDeps,
  pnpm_10,
  nodejs_22,
}:
let
  pnpm' = pnpm_10.override { nodejs = nodejs_22; };
in
{
  package = stdenv.mkDerivation (finalAttrs: {
    pname = "load-balancing-ts";
    version = "v0.0.2";

    # TODO(tatu): Should never do this as, use filesets
    src = ./.;

    nativeBuildInputs = [
      nodejs_22
      pnpm'
      pnpmConfigHook
    ];

    pnpmDeps = fetchPnpmDeps {
      inherit (finalAttrs) pname version src;
      fetcherVersion = 1;
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
