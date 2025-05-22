{
  description = "yyyoink-parse";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = false;
        };

        jailbreakUnbreak = pkg:
          pkgs.haskell.lib.doJailbreak (pkg.overrideAttrs (_: { meta = { }; }));

        packageName = "yyyoink-parse";
      in {
          packages.${packageName} = # (ref:haskell-package-def)
          pkgs.haskellPackages.callCabal2nix packageName self rec {
            # Dependency overrides
          };

        defaultPackage = self.packages.${system}.${packageName};

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.alejandra
            pkgs.haskellPackages.haskell-language-server
            pkgs.haskellPackages.ghcid
            pkgs.haskellPackages.cabal-install
            pkgs.ormolu
          ];

          inputsFrom = builtins.attrValues self.packages.${system};

          shellHook = ''
              . .bashrc
          '';
        };
      }
    );
}
