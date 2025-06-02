{
  writeShellApplication,
  gh,
  pkgs,
  self,
  system
}:
let
  # Can't be arsed to write a HEREDOC in the script :D
  release_message = ''
    # Rolling releases for some projects

    Use at your own peril. These update on each commit.

    ## Artifacts
    **swkotor_mod.dll** - Kotor mod dll
  '';
in
rec {
  package = writeShellApplication {
    name = "gh-release";

    runtimeInputs = [ gh ];

    # FIXME(tatu): Haven't really gone through this with thought, AI generated
    # crap. Maybe works, maybe not.
    # writeShellApplication already sets errexit, nounset and pipefail
    text = ''
      REPO="kuglimon/molokki"
      TAG_NAME="latest"
      RELEASE_NAME="Rolling Release"
      RELEASE_MESSAGE="${release_message}"

      if ! gh auth status &>/dev/null; then
          echo "Error: GitHub CLI is not authenticated. Run 'gh auth login'."
          exit 1
      fi

      if gh release view "$TAG_NAME" --repo "$REPO" 2>/dev/null; then
          echo "Updating existing rolling release..."
          gh release edit "$TAG_NAME" \
              --repo "$REPO" \
              --notes "$RELEASE_MESSAGE" \
              --latest
      else
          echo "Creating new rolling release..."
          gh release create "$TAG_NAME" \
              --repo "$REPO" \
              --title "$RELEASE_NAME" \
              --notes "$RELEASE_MESSAGE" \
              --latest \
              --target main
      fi

      upload_asset() {
        # Upload asset (if exists)
        if [ -f "$1" ]; then
            echo "Uploading asset $1..."
            # ASSET_NAME=$(basename "$1")
            gh release delete-asset "$TAG_NAME" "$1" \
              --repo "$REPO" --yes || true
            gh release upload "$TAG_NAME" "$1" \
              --repo "$REPO" --clobber
            echo "Asset uploaded."
        fi
      }

      # Well, this feels fucked, but it works :D
      upload_asset "${self.outputs.packages.${system}.swkotor-mod}/lib/swkotor_mod.dll"

      echo "Rolling release updated."
    '';
  };

  devShell = pkgs.mkShell {
    buildInputs = [
      pkgs.gh
      package
    ];
  };
}
