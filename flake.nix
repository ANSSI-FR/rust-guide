{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    mdbook0421.url = "nixpkgs/79b3d4bcae8c7007c9fd51c279a8a67acfa73a2a";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
      mdbook0421,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        mdbook-custom = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdBook";
          version = "0.4.52";

          src = pkgs.fetchFromGitHub {
            owner = "hg-anssi";
            repo = pname;
            rev = "c5a35b9296c6d5e48570e30022bd69403050a9f4";
            hash = "sha256-2pUzx5woxGsLu9SMID1u7AufAwa1C7tATln4binTPek=";
          };

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };

          checkFlags = [
            # rustc version 1.87 and after changes output formatting which fails tests
            "--skip=test::failing_tests"
          ];

        });
        mdbook-checklist = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdbook-checklist";
          version = "0.2.0";

          src = ./mdbook-checklist;

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };

        });

        examples = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "examples";
          version = "0.1.0";

          src = ./examples;

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };

        });

        mdbook-code-align = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdbook-code-align";
          version = "0.1.0";

          src = ./mdbook-code-align;

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };

        });

        mdbook-extensions = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdbook-extensions";
          version = "0.1.0";

          src = ./mdbook-extensions;

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };

        });

        mdbook-shiftinclude = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdbook-shiftinclude";
          version = "0.1.0";

          src = pkgs.fetchFromGitHub {
            owner = "daviddrysdale";
            repo = pname;
            tag = "v${version}";
            hash = "sha256-c6rBVeet2wVuW6LFed/TOM1ZevomlzSlSFY4gr5Iv0A=";
          };

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
          };

        });
        runtime = with pkgs; [
          #mdbook0421.legacyPackages.${system}.mdbook
          mdbook-custom
          bash
          mdbook-checklist
          mdbook-shiftinclude
          mdbook-code-align
          mdbook-extensions
          examples
          pandoc
          (aspellWithDicts (
            dicts: with dicts; [
              en
              fr
            ]
          ))
          (python311.withPackages (
            ps: with ps; [
              grammalecte
            ]
          ))
          languagetool
          (texlive.combine {
            inherit (texlive)
              scheme-small
              svg
              transparent
              biblatex
              framed
              ;
          })
        ];
        # mdbook-script = pkgs.writeShellScriptBin "mdbook" ''
        #   ${mdbook-custom}/bin/mdbook
        # '';
        mdbook-app = pkgs.writeShellApplication {
          name = "mdbook";
          runtimeInputs = runtime;
          text = ''
            RUST_BACKTRACE=1 RUST_LOG=info mdbook "$@"
          '';
        };
        shell-app = pkgs.writeShellApplication {
          name = "shell";
          runtimeInputs = runtime ++ [
            # Minimal dependencies for container used in CI
            pkgs.coreutils
            pkgs.gnugrep

            # Used for testing examples
            pkgs.cargo
            pkgs.gcc
          ];
          text = ''
            sh -c "$@"
          '';
        };
      in
      {
        apps."mdbook" = {
          type = "app";
          program = "${mdbook-app}/bin/mdbook";
        };
        apps.default = {
          type = "app";
          program = "${mdbook-app}/bin/mdbook";
        };
        # devShell = pkgs.mkShell {
        #   buildInputs = runtime;
        # };
        packages.default = pkgs.stdenv.mkDerivation {
          name = "rust-guide";
          src = ./.;
          buildPhase = ''
            bash lang.sh fr
            ${mdbook-app}/bin/mdbook build
            bash lang.sh en
            ${mdbook-app}/bin/mdbook build
          '';
          installPhase = ''
            mkdir -p $out
            mv book $out/
          '';
        };
        packages."image" = pkgs.dockerTools.buildImage {
          name = "registry.gitlab.com/anssi-fr/collab/rust-guide/worker";
          tag = "3.0.0";
          config = {
            Entrypoint = [ "${shell-app}/bin/shell" ];
          };
          copyToRoot = [
            pkgs.dockerTools.binSh
            pkgs.dockerTools.usrBinEnv
          ];
        };
      }
    );
}
