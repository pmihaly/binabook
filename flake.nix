{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      pre-commit-hooks,
      treefmt-nix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        devDeps = with pkgs; [
          cargo
          rustc
          websocat
          jq
        ];
        buildDeps = with pkgs; [
          openssl
          pkg-config
        ];

        treefmtEval = treefmt-nix.lib.evalModule pkgs (
          { pkgs, ... }:
          {
            projectRootFile = "flake.nix";
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
          }
        );
      in
      {

        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "binabook";
          version = "0.0.1";
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = buildDeps;
          src = pkgs.lib.cleanSource ./.;
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              nixfmt.enable = true;
              rustfmt.enable = true;
            };
          };
          formatting = treefmtEval.config.build.check self;
        };

        devShell = nixpkgs.legacyPackages.${system}.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          buildInputs = devDeps ++ buildDeps;
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        formatter = treefmtEval.config.build.wrapper;
      }
    );
}
