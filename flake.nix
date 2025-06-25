{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    mdbook0421.url = "nixpkgs/79b3d4bcae8c7007c9fd51c279a8a67acfa73a2a";
  };
  outputs = { self, nixpkgs, utils, mdbook0421 }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        mdbook-checklist = pkgs.rustPlatform.buildRustPackage (finalAttrs: rec {
          pname = "mdbook-checklist";
          version = "0.1.1";

          src = pkgs.fetchFromGitHub {
            owner = "ANSSI-FR";
            repo = pname;
            tag = "v${version}";
            hash = "sha256-7/IRNylcf2sziJQsANc3z5/Pz8Vc3Fe0fB7rp8RL9Y0=";
          };

          cargoLock = { lockFile = "${src}/Cargo.lock"; };

        });
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            mdbook0421.legacyPackages.${system}.mdbook
            bash
            mdbook-checklist
          ];
        };
      });
}
