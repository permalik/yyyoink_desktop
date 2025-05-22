{
  description = "yyyoink_desktop";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    nix-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    nix-unstable,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pname = "yyyoink_desktop";

        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            expat
            fontconfig
            freetype
            freetype.dev
            libGL
            pkg-config
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            wayland
            libxkbcommon
            alejandra
            pre-commit
        ];

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        packages.default = rustPlatform.buildRustPackage {
          name = pname;
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          # buildInputs = [];
          buildPhase = ''
            cargo build --release -p ${pname}
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/${pname} $out/bin/
          '';
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          # buildInputs = with pkgs; [
          #   rust-bin.stable.latest.default
          #   alejandra
          #   pre-commit
          # ];

          RUST_BACKTRACE = 1;

          LD_LIBRARY_PATH =
            builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

          shellHook = ''
            . .bashrc
          '';
        };
      }
    );
}
