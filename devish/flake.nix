{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    make-shell.url = "github:nicknovitski/make-shell";

    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      systems,
      make-shell,
      fenix,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ make-shell.flakeModules.default ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          fenixSys = fenix.packages.${system};
          craneLib = (crane.mkLib pkgs).overrideToolchain fenixSys.minimal.toolchain;
          devish = craneLib.buildPackage {
            src = ./.;
          };
        in
        {
          checks = {
            inherit devish;
          };

          packages = {
            default = devish;
            devish = devish;
          };

          make-shells.default = {
            packages = [
              (fenixSys.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
              fenixSys.rust-analyzer
            ];
          };
        };
    };
}
