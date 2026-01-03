{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Architecture-specific target
        rustTarget = if pkgs.stdenv.isAarch64
                     then "aarch64-unknown-linux-musl"
                     else "x86_64-unknown-linux-musl";

        rustWithTargets = pkgs.rust-bin.stable.latest.default.override {
          targets = [ rustTarget ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # Disable all Nix hardening flags to prevent interference with Cargo builds.
          # These flags are designed for C/C++ and can cause issues with:
          # - MUSL builds (fortify adds glibc-specific functions)
          # - Crates that vendor C libraries (e.g., git2 vendoring libgit2)
          # Rust already provides memory safety, so these hardening flags provide
          # minimal benefit while causing build problems.
          hardeningDisable = [ "all" ];

          buildInputs = [
            # Rust with cross-compilation targets built-in.
            rustWithTargets
            # Shell scripts.
            pkgs.shfmt
            pkgs.shellcheck
            # GitHub Action Workflows.
            pkgs.yamlfmt
            pkgs.actionlint
            # End to end tests.
            pkgs.python313
            pkgs.python313Packages.autopep8
            pkgs.python313Packages.behave
            pkgs.git
            # Deploying.
            pkgs.gh
          ];
        };
      }
    );
}
