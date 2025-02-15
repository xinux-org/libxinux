{
  description = "Xinux'es shared library for package manager";

  inputs = {
    # Latest nixpkgs from nixos
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Xinux Libs for flake automation
    xinux-lib = {
      url = "github:xinux-org/lib";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.xinux-lib.mkFlake {
      inherit inputs;

      src = ./.;
      alias.shells.default = "libxinux";
    };
}
