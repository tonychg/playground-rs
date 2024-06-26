{
  description = "Rust playground";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        pythonFilter = path: _type: builtins.match ".*py$" path != null;
        pythonOrCargo = path: type:
          (pythonFilter path type) || (craneLib.filterCargoSources path type);

        playground-rs = craneLib.buildPackage {
          src = lib.cleanSourceWith {
            src = craneLib.path ./.;
            filter = pythonOrCargo;
          };

          strictDeps = true;

          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          buildInputs = with pkgs; [
            # Add additional build inputs here
            openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          # Additional environment variables can be set directly
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "playground-rs";
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = with pkgs; [
              playground-rs
              openssl
            ];
            pathsToLink = [ "/bin" ];
          };
          config = {
            Cmd = [ "${playground-rs}/bin/playground-rs" ];
          };
        };
      in
      with pkgs;
      {
        checks = {
          inherit playground-rs;
        };

        packages = {
          inherit playground-rs dockerImage;
          default = playground-rs;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = playground-rs;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            rust-analyzer
            git
            go-task
            dive
            pkg-config
            openssl
          ];
        };
      });
}
