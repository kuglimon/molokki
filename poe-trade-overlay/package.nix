{
  pkgs,
}:
{
  package = pkgs.rustPlatform.buildRustPackage {
    pname = "path-of-exile-price-overlay";
    version = "0.1.0";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  };

  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      cargo
    ];

    # Based on: https://github.com/rust-windowing/winit/issues/3603#issuecomment-2016581170
    LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
      with pkgs;
      lib.makeLibraryPath [
        xorg.libxcb
        xorg.libX11
        xorg.libXi
        xorg.libXcursor
        libxkbcommon

        # slint needs for fonts to work
        fontconfig
      ]
    }";
  };
}
