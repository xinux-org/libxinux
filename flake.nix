{
  description = "Xinux'es shared library for package manager";

  inputs = {
    # Latest nixpkgs from nixos
    nixpkgs.url = "github:xinux-org/nixpkgs/nixos-25.11";

    # Xinux Libs for flake automation
    xinux-lib = {
      url = "github:xinux-org/lib/release-25.11";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {self, ...}@inputs:
    inputs.xinux-lib.mkFlake {
      inherit inputs;
      alias.shells.default = "libxinux";
      src = ./.;
      hydraJobs = inputs.self.packages.x86_64-linux;
    };
}
