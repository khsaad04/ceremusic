{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs
    , ...
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.nil
          pkgs.cargo
          pkgs.clippy
          pkgs.rustc
          pkgs.rustfmt
          pkgs.rust-analyzer
          pkgs.yt-dlp
          pkgs.libopus
          pkgs.pkg-config
          pkgs.openssl
        ];
      };
      formatter.${system} = pkgs.nixpkgs-fmt;
    };
}
