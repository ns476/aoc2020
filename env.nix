{ pkgs ? import (builtins.fetchGit {
        url = "https://github.com/NixOS/nixpkgs.git";
        rev = "dabbc5a5603947a6de69104c830a95691921d791";
        ref = "refs/heads/release-20.03";
    }) {}
}:
with pkgs;
let
  rustOverlay = callPackage ./rust.nix {};

  # These are required for manylinux whl support
  # They need to be on LD_LIBRARY_PATH
  linuxWheelPackages = [
        binutils
        gcc
        glib
        ncurses
        xorg.libICE
        xorg.libSM
        xorg.libX11
        xorg.libXext
        xorg.libXrender
  ];

  linuxPackages = linuxWheelPackages ++ [
    glibcLocales
    inotify-tools
  ];

  macPackages = [];

  osPackages =
    if stdenv.isLinux then linuxPackages
    else if stdenv.isDarwin then macPackages
    else abort "Only Mac and Linux are supported";

  rustWithExtensions = rustOverlay.rust.override {
      extensions = [
        "rust-src"
        "rust-analyzer-preview"
        "clippy-preview"
        "rustfmt-preview"
      ];
    };

  buildInputs = [
    (yarn.override { nodejs = nodejs-12_x; })
    bash
    git
    gitAndTools.hub
    gnumake
    httpie
    jq
    less
    lldb
    moreutils
    nodejs-12_x
    openssl
    parallel
    pkgconfig
    plantuml
    poetry
    postgresql_12
    pythonPackages.pygments
    rustWithExtensions
    rustOverlay.cargo
    sqlite
    yq
    python39
  ] ++ osPackages;

  libPath = stdenv.lib.makeLibraryPath(buildInputs);
in
  mkShell {
    buildInputs = buildInputs;

    shellHook = if stdenv.isLinux then
      "export LD_LIBRARY_PATH=\"" +
      "$(for i in $(cat $NIX_CC/nix-support/cc-ldflags | grep --only-matching \"/.*\"); do echo -n $i:; done)" +
      "${libPath}\""
    else
      "export DYLD_FALLBACK_LIBRARY_PATH=\"" +
      "$(for i in $(cat $NIX_CC/nix-support/cc-ldflags | grep --only-matching \"/.*\"); do echo -n $i:; done)" +
      "${libPath}\"";
  }
