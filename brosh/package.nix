{
  pkgs,
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

  run-all-tests = pkgs.writeShellApplication {
    name = "run-tests.sh";

    runtimeInputs = [
    ];

    text = builtins.readFile ./devtools/run-tests.sh;
  };
in
{
  package = pkgs.rustPlatform.buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;
    nativeBuildInputs = with pkgs; [
      pkg-config
      rustPlatform.bindgenHook

      # Enable to debug failing builds
      # breakpointHook
    ];

    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

    checkPhase = ''
      cargo test
    '';
  };

  devShell = pkgs.mkShell {
    buildInputs = [
    ];

    packages = with pkgs; [
      run-all-tests

      # Following dependencies used for building bash
      # Build tools
      autoconf
      automake
      m4
      bison
      flex
      gnumake
      gcc
      pkg-config

      # Required libraries
      readline
      ncurses
      gettext

      # Documentation
      texinfo
      groff

      # Testing/debugging
      gdb
      valgrind
      diffutils

      # generates compile_commands.json for clangd
      bear

      # Useful utilities
      git
      less
    ];

    shellHook = ''
      echo "GNU Bash development environment"
      echo ""
      echo "Build instructions:"
      echo "  # If configure doesn't exist (git clone):"
      echo "  autoconf"
      echo ""
      echo "  # Build with LSP support (generates compile_commands.json):"
      echo "  ./configure"
      echo "  bear -- make -j\$(nproc)"
      echo ""
      echo "  # Or plain build:"
      echo "  ./configure && make -j\$(nproc)"
      echo ""
      echo "  # Run tests:"
      echo "  make tests"
      echo ""
      echo "Note: Do NOT use autoreconf - bash has a custom build system."
      echo ""
    '';

    # Ensure the linker can find readline
    NIX_LDFLAGS = "-L${pkgs.readline}/lib";
    NIX_CFLAGS_COMPILE = "-I${pkgs.readline}/include";
  };
}
