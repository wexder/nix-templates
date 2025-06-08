{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    make-shell.url = "github:nicknovitski/make-shell";

    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, systems, make-shell, fenix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ make-shell.flakeModules.default ];
      systems = [ "x86_64-linux" "aarch64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          fenixSys = fenix.packages.${system};
        in
        {
          make-shells.default = {
            checks = self.checks.${system};

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

