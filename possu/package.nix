{
  pkgs,
  crane,
  fenix,
}:
let
  craneLib = crane.mkLib pkgs;
  src = craneLib.cleanCargoSource ./.;

  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;
    strictDeps = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  possu = craneLib.buildPackage (
    commonArgs
    // {
      pname = "possu";
      cargoExtraArgs = "-p possu";
    }
  );
in
rec {
  package = possu;

  checks = {
    # Build the crate as part of `nix flake check` for convenience
    inherit possu;

    # Run clippy (and deny all warnings) on the crate source,
    # again, reusing the dependency artifacts from above.
    #
    # Note that this is done as a separate derivation so that
    # we can block the CI if there are issues here, but not
    # prevent downstream consumers from building our crate by itself.
    clippy = craneLib.cargoClippy (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      }
    );

    doc = craneLib.cargoDoc (
      commonArgs
      // {
        inherit cargoArtifacts;
      }
    );

    # Check formatting
    fmt = craneLib.cargoFmt {
      inherit src;
    };

    toml-fmt = craneLib.taploFmt {
      src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
      # taplo arguments can be further customized below as needed
      # taploExtraArgs = "--config ./taplo.toml";
    };
  };

  devShell = craneLib.devShell {
    # Inherit inputs from checks.
    checks = checks;
  };
}
