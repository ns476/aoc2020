{ pkgs ? import (builtins.fetchGit {
        url = "https://github.com/NixOS/nixpkgs.git";
        rev = "dabbc5a5603947a6de69104c830a95691921d791";
        ref = "refs/heads/release-20.03";
    }) {}
}:
with pkgs;
let
  stableRust = callPackage ./rust.nix {};

  macPackages = [];

  rustWithExtensions = stableRust.rust.override {
      extensions = [
        "rust-src"
        "rls-preview"
        "clippy-preview"
        "rustfmt-preview"
      ];
    };

  buildInputs = [
    bash
    git
    rustWithExtensions
    stableRust.cargo
  ];

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
