{
  pkgs,
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

  autosavevim = pkgs.writeShellScriptBin "autosavevim" ''
    # Used in integration tests to avoid process hanging on vim. Saves the
    # file and exists.
    echo "in path $PATH"
    ${pkgs.neovim}/bin/nvim -i NONE -u NONE -c ":wq" "$@"
  '';

  test-tmux = pkgs.writeShellScriptBin "tmux" ''
    # Used in integration tests to spawn a custom tmux-server. Users might
    # have their own servers running and we don't want to collide.
    echo "in path $PATH"
    ${pkgs.tmux}/bin/tmux -L rojekti-it "$@"
  '';
in
pkgs.rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustPlatform.bindgenHook

    # Enable to debug failing builds
    # breakpointHook
  ];

  nativeCheckInputs = [
    autosavevim
    test-tmux
  ];

  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  checkPhase = ''
    cargo test
  '';
}
