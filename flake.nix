{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          src = ./.;
        in
        {
          packages = {
            default = pkgs.rustPlatform.buildRustPackage {
              name = "sway-switch-outputs";
              src = src;
              cargoLock = {
                lockFile = "${src}/Cargo.lock";
              };
            };
          };
          devShells.default = pkgs.mkShell {
            buildInputs = [
              self.packages.${system}.default.buildInputs
              pkgs.cargo
            ];
          };
          apps = {
            default = flake-utils.lib.mkApp {
              drv = self.packages.${system}.default;
            };
          };
        }
      );
}
