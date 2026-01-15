# SPDX-FileCopyrightText: 2026 Mozilla
# SPDX-FileContributor: Nicolas Qiu Guichard <nicolas.guichard@kdab.com>
#
# SPDX-License-Identifier: Apache-2.0 OR MIT
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
  }: (
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };

      craneLib = crane.mkLib pkgs;

      sourceFilter = path: type:
        (builtins.match ".*\\.(md|wasm|tpl)$" path != null) || (craneLib.filterCargoSources path type);
      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = sourceFilter;
        name = "source";
      };
      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src;
      };
      wasm-snip = craneLib.buildPackage {
        inherit src cargoArtifacts;
        nativeCheckInputs = with pkgs; [
          cargo-readme
        ];
      };
    in {
      packages.${system} = {
        inherit wasm-snip;
        default = wasm-snip;
      };

      apps.${system} = {
        wasm-snip = {
          type = "app";
          program = "${wasm-snip}/bin/wasm-snip";
          meta.description = "wasm-snip replaces a Wasm function's body with an unreachable instruction.";
        };
        default = self.apps.${system}.wasm-snip;
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [wasm-snip];

        packages = with pkgs; [
          rust-analyzer
          rr
          rustfmt
          clippy
          reuse
          taplo
        ];
      };

      checks.${system} = {
        inherit wasm-snip;

        clippy = craneLib.cargoClippy {
          inherit src cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };

        fmt = craneLib.cargoFmt {
          inherit src;
        };

        toml-fmt = craneLib.taploFmt {
          src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
        };

        reuse = pkgs.runCommand "check-reuse" {} ''
          cd ${self}
          ${pkgs.reuse}/bin/reuse lint
          touch $out
        '';

        nix-fmt = pkgs.runCommand "check-nix-fmt" {} ''
          ${self.formatter.${system}}/bin/${self.formatter.${system}.NIX_MAIN_PROGRAM} -c ${self}/flake.nix
          touch $out
        '';
      };

      formatter.${system} = pkgs.alejandra;
    }
  );
}
