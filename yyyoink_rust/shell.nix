let
  flake = import ./flake.nix;
in
  flake.outputs.devShell
