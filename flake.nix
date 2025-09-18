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

        });
        mdbook-checklist = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdbook-checklist";
          version = "0.1.1";

          src = pkgs.fetchFromGitHub {
            owner = "ANSSI-FR";
            repo = pname;
            tag = "v${version}";
            hash = "sha256-7/IRNylcf2sziJQsANc3z5/Pz8Vc3Fe0fB7rp8RL9Y0=";
          };

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
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            #mdbook0421.legacyPackages.${system}.mdbook
            mdbook-custom
            bash
            mdbook-checklist
            mdbook-shiftinclude
            mdbook-code-align
          ];
        };
      }
    );
}
