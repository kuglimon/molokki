{
  pkgs,
  crane,
  fenix,
  system
}:
let
  craneLib = crane.mkLib pkgs;
  src = craneLib.cleanCargoSource ./krangle-api;

  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;
    strictDeps = true;

    buildInputs = [
      # Add additional build inputs here
    ];

    # Additional environment variables can be set directly
    # MY_CUSTOM_VAR = "some value";
  };

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  krangle-api-crate = craneLib.buildPackage (commonArgs // {
    inherit cargoArtifacts;
  });
in
rec {
  package = krangle-api-crate;

  checks = {
    # Build the crate as part of `nix flake check` for convenience
    inherit krangle-api-crate;

    # Run clippy (and deny all warnings) on the crate source,
    # again, reusing the dependency artifacts from above.
    #
    # Note that this is done as a separate derivation so that
    # we can block the CI if there are issues here, but not
    # prevent downstream consumers from building our crate by itself.
    my-crate-clippy = craneLib.cargoClippy (commonArgs // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    });

    my-crate-doc = craneLib.cargoDoc (commonArgs // {
      inherit cargoArtifacts;
    });

    # Check formatting
    my-crate-fmt = craneLib.cargoFmt {
      inherit src;
    };

    my-crate-toml-fmt = craneLib.taploFmt {
      src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
      # taplo arguments can be further customized below as needed
      # taploExtraArgs = "--config ./taplo.toml";
    };
  };

  devShell = craneLib.devShell {
    # Inherit inputs from checks.
    checks = checks;

    # Extra inputs can be added here; cargo and rustc are provided by default.
    packages = [
      pkgs.k3d
      pkgs.docker
      pkgs.kubectl
    ];

    shellHook = ''
      echo "Kubernetes Operator POC" | ${pkgs.figlet}/bin/figlet
      echo
      echo "Welcome to the Kubernetes Operator POC dev shell!"
      echo "To start a cluster, run:"
      echo "  k3d cluster create --config k8s/k3d-config.yaml"
      echo "To stop a cluster, run:"
      echo "  k3d cluster stop dev"
      echo "To access your cluster:"
      echo "  kubectl cluster-info"
    '';
  };
}
