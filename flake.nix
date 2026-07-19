{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, fenix, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        target = "x86_64-unknown-linux-musl";
        toolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.stable.rustc
          fenix.packages.${system}.stable.cargo
          fenix.packages.${system}.targets.${target}.stable.rust-std
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        meta = builtins.fromTOML (builtins.readFile "${self}/meta.toml");

        protoTarball = pkgs.fetchurl (let
          version = meta.proto.version;
          hash = meta.proto.hash;
        in {
          url = "https://augustomegener.github.io/kernel-paniks-proto/augustomegener/kernel-paniks-proto/${version}/kernel-paniks-proto-${version}.tar";
          sha256 = hash;
        });

        rustSrc = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = craneLib.filterCargoSources;
        };

        src = pkgs.runCommand "orchestrator-src" { } ''
          mkdir -p $out
          cp -r ${rustSrc}/. $out/
          tar -xf ${protoTarball} -C $out
        '';

        commonArgs = {
          inherit src;
          strictDeps = true;
          CARGO_BUILD_TARGET = target;
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          nativeBuildInputs = [ pkgs.protobuf ];
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        packages.default = bin;
        devShells.default = craneLib.devShell {
          packages = [ toolchain pkgs.protobuf ];
        };
      });
}
