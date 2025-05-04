{
  pkgs,
  crane,
  fenix,
  symlinkJoin,
  runCommandLocal,
  system
}:
let
  inherit (pkgs) lib;

  craneLib = crane.mkLib pkgs;
  src = craneLib.cleanCargoSource ./.;

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

  # Build *just* the cargo dependencies (of the entire workspace),
  # so we can reuse all of that work (e.g. via cachix) when running in CI
  # It is *highly* recommended to use something like cargo-hakari to avoid
  # cache misses when building individual top-level-crates  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  individualCrateArgs = commonArgs // {
    inherit cargoArtifacts;
    inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
    # NB: we disable tests since we'll run them all via cargo-nextest
    doCheck = false;
  };

  fileSetForCrate = crate: lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      (craneLib.fileset.commonCargoSources ./crates/krangle-api)
      (craneLib.fileset.commonCargoSources ./crates/krangle-operator)
      (craneLib.fileset.commonCargoSources crate)
    ];
  };

  # Build the top-level crates of the workspace as individual derivations.
  # This allows consumers to only depend on (and build) only what they need.
  # Though it is possible to build the entire workspace as a single derivation,
  # so this is left up to you on how to organize things
  #
  # Note that the cargo workspace must define `workspace.members` using wildcards,
  # otherwise, omitting a crate (like we do below) will result in errors since
  # cargo won't be able to find the sources for all members.
  krangle-api-crate = craneLib.buildPackage (individualCrateArgs // {
    pname = "krangle-api";
    cargoExtraArgs = "-p krangle-api";
    src = fileSetForCrate ./crates/krangle-api;
  });

  krangle-operator-crate = craneLib.buildPackage (individualCrateArgs // {
    pname = "krangle-operator";
    cargoExtraArgs = "-p krangle-operator";
    src = fileSetForCrate ./crates/krangle-operator;
  });

  krangle-api-image = pkgs.dockerTools.buildLayeredImage {
    name = "krangle-api";
    tag = "latest";
    contents = [ krangle-api-crate ];
    config.Cmd = [ "/bin/krangle-api" ];
  };

  joined = symlinkJoin {
    name = "kube-operator-poc-rs";
    paths = [
      krangle-api-crate
      krangle-operator-crate
    ];
  };
in
rec {
  package = runCommandLocal "kube-operator-poc-rs" {} ''
    mkdir -p $out/images

    cp -rs ${joined}/* $out
    cp -rs ${krangle-api-image} $out/images
  '';

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
