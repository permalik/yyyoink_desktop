{
  description = "zig_sandbox";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    nix-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    zig-overlay.url = "github:mitchellh/zig-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    nix-unstable,
    flake-utils,
    zig-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        zigPackage = zig-overlay.packages.${system}."0.14.0";
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.mkShell {
            name = "zig_sandbox";
            packages = with pkgs; [
                alejandra
                pre-commit
            ];
            nativeBuildInputs = [
                zigPackage
            ];

            shellHook = with pkgs; ''
                # Source .bashrc
                . .bashrc
            '';
        };

        packages.default = pkgs.stdenv.mkDerivation {
            name = "zig_sandbox";
            src = ./.;

            XDG_CACHE_HOME = "${placeholder "out"}";

            # buildInputs = [];
            buildPhase = ''
                ${zigPackage}/bin/zig build
            '';

            installPhase = ''
                ${zigPackage}/bin/zig build install --prefix $out
                rm -rf $out/zig
            '';
        };
      }
    );
}
