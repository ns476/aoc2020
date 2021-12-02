{
    stdenv,
    pkgs,
    fetchFromGitHub,
    pkgconfig,
    openssl
}:
let src = fetchFromGitHub {
      owner = "oxalica";
      repo = "rust-overlay";
      rev = "4129ba9dacaf069e5f85956fdc1272324b2908c9";
      sha256 = "11wqrg86g3qva67vnk81ynvqyfj0zxk83cbrf0p9hsvxiwxs8469";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
(rustChannelOf { date = "2021-10-27"; channel = "nightly"; })
