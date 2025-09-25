{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            clippy
            pkg-config
          ];

          buildInputs = with pkgs; [
            clang
          ];

          packages = with pkgs; [
            rust-analyzer
            (rustfmt.override { asNightly = true; })
            just
            cargo-audit
            cargo-nextest
            cargo-machete
            cargo-watch
            cargo-rdme
            cargo-workspaces
          ];

          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        };
      }
    );
}
