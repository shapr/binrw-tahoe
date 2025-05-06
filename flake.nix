{
  description = "binrw_tahoe";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    crane.url = "github:ipetkov/crane";
    rustOverlay.url = "github:oxalica/rust-overlay";
    devshell.url = "github:numtide/devshell";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    rustOverlay,
    devshell,
    advisory-db,
  }: let
      forAllSystems = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ] (system: function rec {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                (import rustOverlay)
                devshell.overlays.default
              ];
            };

            rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
              toolchain.default.override {
                extensions = ["rust-src"];
              });

            craneLib = crane.lib.${system}.overrideToolchain rustToolchain;

            src = craneLib.cleanCargoSource ./.;

            craneCommon = {
              inherit src;
              RUSTFLAGS = [
                # Lint groups
                ["-D" "clippy::correctness"]
                ["-D" "clippy::complexity"]
                ["-D" "clippy::perf"]
                ["-D" "clippy::style"]
                ["-D" "clippy::nursery"]
                ["-D" "clippy::pedantic"]
                # Allowed by default
                ["-D" "clippy::cognitive_complexity"]
                ["-D" "clippy::expect_used"]
                ["-D" "clippy::unwrap_used"]
                ["-D" "clippy::print_stderr"]
                ["-D" "clippy::print_stdout"]
                ["-D" "clippy::pub_use"]
                ["-D" "clippy::redundant_closure_for_method_calls"]
                ["-D" "clippy::single_char_lifetime_names"]
                ["-D" "clippy::str_to_string"]
                ["-D" "clippy::string_to_string"]
                ["-D" "clippy::unwrap_in_result"]
                ["-D" "clippy::wildcard_enum_match_arm"]
                # Allow certain rules
                ["-A" "clippy::missing_errors_doc"]
                ["-A" "clippy::module_name_repetitions"]
                # No warnings
                # ["-D" "warnings"]
              ];
            };

            cargoArtifacts = craneLib.buildDepsOnly craneCommon;

            binrw_tahoe = craneLib.buildPackage (craneCommon
              // {
                inherit cargoArtifacts;
              });
            });
    in {
      checks = forAllSystems ({
        binrw_tahoe,
        craneLib,
        craneCommon,
        cargoArtifacts,
        src, ...
      }: {
        inherit binrw_tahoe;

        binrw_tahoe-clippy = craneLib.cargoClippy (craneCommon
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets";
          });

        # Check formatting
        binrw_tahoe-fmt = craneLib.cargoFmt {
          inherit src;
        };

        # Audit dependencies
        binrw_tahoe-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };
      });

      formatter = forAllSystems ({ pkgs, ... }: pkgs.alejandra);

      packages = forAllSystems ({ binrw_tahoe, ... }: {
        inherit binrw_tahoe;
        default = binrw_tahoe;
      });

      apps = forAllSystems ({system, ...}: {
        binrw_tahoe = {
          type = "app";
          program = "${self.packages.${system}.binrw_tahoe}/bin/binrw_tahoe";
        };
        default = self.apps.${system}.binrw_tahoe;
      });

      devShells = forAllSystems ({
        craneCommon,
        rustToolchain,
        pkgs,
        ...
      }: {
        default = pkgs.devshell.mkShell {
        commands = let
          categories = {
            hygiene = "hygiene";
            development = "development";
          };
        in [
          {
            help = "Check rustc and clippy warnings";
            name = "check";
            command = ''
              set -x
              cargo check --all-targets
              cargo clippy --all-targets
            '';
            category = categories.hygiene;
          }
          {
            help = "Automatically fix rustc and clippy warnings";
            name = "fix";
            command = ''
              set -x
              cargo fix --all-targets --allow-dirty --allow-staged
              cargo clippy --all-targets --fix --allow-dirty --allow-staged
            '';
            category = categories.hygiene;
          }
          {
            help = "Run cargo in watch mode";
            name = "watch";
            command = "cargo watch";
            category = categories.development;
          }
        ];

        imports = ["${devshell}/extra/language/rust.nix"];
        language.rust = {
          packageSet = rustToolchain;
          tools = ["rustc"];
          enableDefaultToolchain = false;
        };

        devshell = {
          name = "binrw_tahoe-devshell";
          packages = with pkgs; [
            # Rust build inputs
            clang
            coreutils

            # LSP's
            rust-analyzer

            # Tools
            cargo-watch
            rustToolchain
            alejandra
          ];
        };

        env = [
          {
            name = "RUSTFLAGS";
            eval = "\"${builtins.toString craneCommon.RUSTFLAGS}\"";
          }
        ];
        };
      });
    };
}
