{
  pkgs,
  lib,
  config,
  ...
}:
{
  # https://devenv.sh/packages/
  packages = [
    pkgs.trunk
    pkgs.wasm-bindgen-cli
    pkgs.wrangler
    pkgs.binaryen
  ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "stable";
      targets = [ "wasm32-unknown-unknown" ];
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
