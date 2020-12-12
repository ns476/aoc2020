{
    stdenv,
    pkgs,
    fetchFromGitHub,
    pkgconfig,
    openssl
}:
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      # commit from: 2019-05-15
      rev = "efda5b357451dbb0431f983cca679ae3cd9b9829";
      sha256 = "11wqrg86g3qva67vnk81ynvqyfj0zxk83cbrf0p9hsvxiwxs8469";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
(rustChannelOf { date = "2020-08-27"; channel = "stable"; })
