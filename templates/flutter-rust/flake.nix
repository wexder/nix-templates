{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    stable-nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    make-shell.url = "github:nicknovitski/make-shell";

    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix/monthly";
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
      fenix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ make-shell.flakeModules.default ];
      systems = [
        "aarch64-linux"
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
          systemImageType = "default";
          stable_pkgs = stable-nixpkgs.legacyPackages.${system};
          fenixSys = fenix.packages.${system};
          androidEnv = pkgs.androidenv.override { licenseAccepted = true; };
          androidComp = (
            androidEnv.composeAndroidPackages {
              cmdLineToolsVersion = "8.0";
              includeNDK = true;
              # we need some platforms
              buildToolsVersions = [
                "27.0.1"
                "36.1.0"
              ];
              platformVersions = [
                "30"
                "34"
                "35"
                "36"
              ];
              # we need an emulator
              includeEmulator = true;
              includeSystemImages = true;
              systemImageTypes = [
                systemImageType
                # "google_apis"
              ];
              abiVersions = [
                "x86"
                "x86_64"
                "armeabi-v7a"
                "arm64-v8a"
              ];
              cmakeVersions = [ "3.10.2" ];
            }
          );
          android_sdk = (pkgs.android-studio.withSdk androidComp.androidsdk);
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
            ];
            config = {
              allowUnfree = true;
              android_sdk.accept_license = true;
              allowUnsupportedSystem = true;
            };
          };

          packages.android-emulator = androidEnv.emulateApp {
            name = "emulate-MyAndroidApp";
            platformVersion = "35";
            abiVersion = "arm64-v8a"; # armeabi-v7a, mips, x86_64, arm64-v8a
            systemImageType = systemImageType;
            configOptions = {
              "skin.name" = "480x854";
            };
          };

          make-shells.default = {
            env = {
              ANDROID_SDK = "${androidComp.androidsdk}";
              GRADLE_PATH = "${pkgs.gradle}";
              CHROME_EXECUTABLE = "${stable_pkgs.chromium}/bin/chromium";
              FLUTTER_ROOT = "${pkgs.flutter}";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
              LD_LIBRARY_PATH = pkgs.lib.strings.concatStrings (
                pkgs.lib.strings.intersperse ":" [
                  "$LD_LIBRARY_PATH"
                ]
              );
            };

            packages = [
              pkgs.flutter
              pkgs.flutter_rust_bridge_codegen
              pkgs.glibc_multi.dev
              pkgs.llvmPackages.libcxxClang
              pkgs.llvmPackages.llvm
              pkgs.cargo-ndk

              (fenixSys.combine [
                fenixSys.complete.cargo
                fenixSys.complete.clippy
                fenixSys.complete.rust-src
                fenixSys.complete.rustc
                fenixSys.complete.rustfmt
                fenixSys.targets.aarch64-linux-android.latest.rust-std
                fenixSys.targets.i686-linux-android.latest.rust-std
                fenixSys.targets.armv7-linux-androideabi.latest.rust-std
                fenixSys.targets.x86_64-linux-android.latest.rust-std
              ])
              fenixSys.rust-analyzer

              android_sdk
              stable_pkgs.chromium
            ];
          };
        };
    };
}
