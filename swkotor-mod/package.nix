{
  pkgs,
  crane,
  fenix,
  writeTextFile,
  symlinkJoin,
  # XXX(tatu): Am I supposed to pass system like this? Maybe this should be
  # higher up
  system
}:
let
  # Stolen from: https://github.com/deltachat/deltachat-core-rust/blob/8dcd8aa69d600ab5847bd1c38a08aee38af7c844/flake.nix#L159
  # Get rid of MCF Gthread library.
  # See <https://github.com/NixOS/nixpkgs/issues/156343>
  # and <https://discourse.nixos.org/t/statically-linked-mingw-binaries/38395>
  # for details.
  #
  # Use DWARF-2 instead of SJLJ for exception handling.
  winCC = pkgs.pkgsCross.mingw32.buildPackages.wrapCC (
    (pkgs.pkgsCross.mingw32.buildPackages.gcc-unwrapped.override
      ({
        threadsCross = {
          model = "win32";
          package = null;
        };
      })).overrideAttrs (oldAttr: {
      configureFlags = oldAttr.configureFlags ++ [
        "--disable-sjlj-exceptions --with-dwarf2"
      ];
    })
  );

  toolchain = with fenix.packages.${system};
    combine [
      minimal.rustc
      minimal.cargo
      targets.i686-pc-windows-gnu.latest.rust-std
    ];

  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

  swkotor-mod = craneLib.buildPackage rec {
    src = craneLib.cleanCargoSource ./.;

    strictDeps = true;
    doCheck = false;

    CARGO_BUILD_TARGET = "i686-pc-windows-gnu";

    TARGET_CC   = "${winCC}/bin/${winCC.targetPrefix}cc";

    CARGO_BUILD_RUSTFLAGS = [
      "-C"
      "linker=${TARGET_CC}"
    ];

    # fixes issues related to openssl
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

    CC = "${winCC}/bin/${winCC.targetPrefix}cc";
    LD = "${winCC}/bin/${winCC.targetPrefix}cc";

    depsBuildBuild = with pkgs; [
      pkgsCross.mingw32.stdenv.cc
      pkgsCross.mingw32.windows.pthreads
    ];
  };

  # Helper script for launching kotor directly through steam with correct dll in
  # place and all. Note that this script is impure, it assumes you have steam
  # installed and kotor installed.
  #
  # We are using 'writeTextFile' because the bash environment nix would run this
  # in isn't usable in steam context. It's likely fixable, but I'm just cutting
  # corners here. I couldn't get the automation working either.
  #
  # To use this set this script as the launch option like so:
  #   <path_to_repo>/.result/bin/run-swkotor-mod %command%
  runSWKotorMod = writeTextFile {
    name = "run-swkotor-mod";
    text = ''
      #!/usr/bin/env bash
      LOG_FILE="$HOME/.steam-debugger.log"

      # Redirect IO to file
      exec >> "$LOG_FILE"

      # If we have no command then ask steam to start the game using this script
      if [ -z "$1" ]; then
        # Get our path and pass it to steam
        SCRIPT_PATH="$(dirname $(realpath -s $0))/run-swkotor-mod"
        URL_SAFE_COMMAND="$(printf %s "$SCRIPT_PATH %command%"|${pkgs.jq}/bin/jq -sRr @uri)"

        echo "Make steam start kotor with arguments '$SCRIPT_PATH %command%'"

        # FIXME(tatu): This does not work
        # Start our kotor and make launch options point to this script.
        # steam "steam://run/32370//$URL_SAFE_COMMAND/"
        exit 0
      fi

      echo "Starting debugging steam game with command:"
      echo "$@"

      # Force steam to load our modified dll
      # FIXME(tatu): Can this be an absolute path?
      # export WINEDLLOVERRIDES="dinput8.dll=n"

      # Exposes debug tools like winedbg in /tmp
      export PROTON_LOG=1
      export STEAM_COMPAT_LAUNCHER_SERVICE=proton

      # Copy our built dll to the game install directory under steam.
      cp -rf "${swkotor-mod}/lib/swkotor_mod.dll" ~/.local/share/Steam/steamapps/common/swkotor/dinput8.dll

      echo "DLL copied, starting kotor"

      # MANGOHUD_CONFIG=fps_limit=60 mangohud
      "$@"
    '';
    executable = true;
    destination = "/bin/run-swkotor-mod";
  };
in
symlinkJoin {
  name = "swkotor-mod";
  paths = [ swkotor-mod runSWKotorMod ];
}
