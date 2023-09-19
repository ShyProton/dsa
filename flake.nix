{
  description = "NES Emulator built with Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    nix-filter.url = "github:numtide/nix-filter";
    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    nix-filter,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = ["wasm32-unknown-unknown"];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = nix-filter.lib.filter {
        root = craneLib.path ./.;
        include = [
          "src"
          "site"
          "Cargo.lock"
          (nix-filter.lib.matchExt "rs")
          (nix-filter.lib.matchExt "toml")
          (nix-filter.lib.matchExt "html")
        ];
      };

      # Common arguments can be set here to avoid repeating them later
      commonArgs = {
        inherit src;

        buildInputs =
          [
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

        # Additional environment variables can be set directly
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
      };

      craneLibLLvmTools =
        craneLib.overrideToolchain
        (fenix.packages.${system}.complete.withComponents [
          "cargo"
          "llvm-tools"
          "rustc"
        ]);

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          # Cannot perform cargo test on a wasm build.
          doCheck = false;
        });

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      crate = craneLib.buildTrunkPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });

      dockerImage = pkgs.dockerTools.buildImage {
        name = "nes_emulator";
        tag = "latest";
        copyToRoot = [crate];
        config = {
          Cmd = ["${crate}/bin/nes_emulator"];
        };
      };

      serve-app = pkgs.writeShellScriptBin "serve-app" ''
        ${pkgs.python3Minimal}/bin/python3 -m http.server --directory ${crate} 8000
      '';
    in {
      checks = {
        # Build the crate as part of `nix flake check` for convenience
        inherit crate;

        # Run clippy (and deny all warnings) on the crate source,
        # again, resuing the dependency artifacts from above.
        #
        # Note that this is done as a separate derivation so that
        # we can block the CI if there are issues here, but not
        # prevent downstream consumers from building our crate by itself.
        crate-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used ";
          });

        crate-doc = craneLib.cargoDoc (commonArgs
          // {
            inherit cargoArtifacts;
          });

        # Check formatting
        crate-fmt = craneLib.cargoFmt {
          inherit src;
        };

        # Audit dependencies
        crate-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };
      };

      packages = {
        inherit crate dockerImage;
        default = crate;
        crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs
          // {
            inherit cargoArtifacts;
          });
      };

      apps.default = flake-utils.lib.mkApp {
        drv = serve-app;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks.${system};

        # Additional dev-shell environment variables can be set directly
        # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs;
          [
            bacon
            cargo-modules
            dive
          ]
          ++ [rustToolchain];
      };
    });
}
