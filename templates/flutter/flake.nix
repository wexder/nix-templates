{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    stable-nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    make-shell.url = "github:nicknovitski/make-shell";
    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs";
      # url = "github:tadfisher/android-nixpkgs/stable";
      # url = "github:tadfisher/android-nixpkgs/beta";
      # url = "github:tadfisher/android-nixpkgs/preview";
      # url = "github:tadfisher/android-nixpkgs/canary";

      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      stable-nixpkgs,
      flake-parts,
      systems,
      make-shell,
      android-nixpkgs,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ make-shell.flakeModules.default ];
      systems = [
        "x86_64-linux"
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
          stable_pkgs = stable-nixpkgs.legacyPackages.${system};
          android_sdk = android-nixpkgs.sdk.${system} (
            sdkPkgs: with sdkPkgs; [
              cmdline-tools-latest
              build-tools-34-0-0
              platform-tools
              platforms-android-34
              emulator
            ]
          );
        in
        {
          make-shells.default = {
            env.CHROME_EXECUTABLE = "${stable_pkgs.chromium}/bin/chromium";

            packages = [
              pkgs.flutter
              android_sdk
              stable_pkgs.chromium
            ];
          };
        };
    };
}
