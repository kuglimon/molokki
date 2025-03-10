# FIXME(tatu): FIX this dependency duplication. What I'd really like to have is
# shell inherit all the dependencies from the package.nix and then just add the
# extra deps on top of those.
{
  pkgs ? import <nixpkgs> { },
}:
let
  # FIXME(tatu): See top comment
  autosavevim = pkgs.writeShellScriptBin "autosavevim" ''
    # Used in integration tests to avoid process hanging on vim. Saves the
    # file and exists.
    echo "in path $PATH"
    ${pkgs.neovim}/bin/nvim -i NONE -u NONE -c ":wq" "$@"
  '';

  # FIXME(tatu): See top comment
  test-tmux = pkgs.writeShellScriptBin "tmux" ''
    # Used in integration tests to spawn a custom tmux-server. Users might
    # have their own servers running and we don't want to collide.
    echo "in path $PATH"
    ${pkgs.tmux}/bin/tmux -L rojekti-it "$@"
  '';
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # FIXME(tatu): This is not optimal, this causes recompilation.
    # `runPackage` should be used instead?
    # FIXME(tatu): These are not working, not visible in cargo path
    #
    # Lord fucking mighty I love nix. Instead of fucking up users local
    # neovim with some random ass version required for tests, we can just
    # pass a special version to the command that needs it!
    (cargo.overrideAttrs (oldAttrs: {
      buildInputs = oldAttrs.buildInputs ++ [
        autosavevim
        test-tmux
      ];
    }))

    alejandra
    autosavevim
    test-tmux

    # tmuxinator for testing feature parity
    (ruby.withPackages (ps: with ps; [ tmuxinator ]))
  ];
}
