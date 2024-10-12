{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in {
      devShell = pkgs.mkShell rec {
      nativeBuildInputs = with pkgs; [
	pkg-config
      ];
      buildInputs = with pkgs; [
        libxkbcommon
        libGL

	# WINIT_UNIX_BACKEND=wayland
	wayland

	# WINIT_UNIX_BACKEND=x11
	xorg.libXcursor
	xorg.libXrandr
	xorg.libXi
	xorg.libX11
	vulkan-loader
      ];
      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
    };


  });
}
