{
    description = "6502 Emulator";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = {self, nixpkgs}:
        let
            system = "x86_64-linux";
            pkgs = nixpkgs.legacyPackages.${system};
        in 
        {
            devShells.${system}.default = pkgs.mkShell {
                buildInputs = with pkgs; [
                    python313
                    rust
                    make
                    cc65
                ];
            };
        };

}