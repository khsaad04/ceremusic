{
  description = "ceremusic devshell";
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };
  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [yt-dlp libopus];
      nativeBuildInputs = with pkgs; [
        cmake
        openssl
        pkg-config
      ];
    };
  };
}
