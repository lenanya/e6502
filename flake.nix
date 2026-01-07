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
                packages = with pkgs; [
                    cargo
                    cc65
                    gnumake
                    cmake
                    xorg.libX11
                    xorg.libXcursor
                    xorg.libXrandr
                    xorg.libXinerama
                    xorg.libXi
                    xorg.libXext
                    libGL
                    libclang
                ];
                shellHook = ''
                    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
                    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.libGL ]}:$LD_LIBRARY_PATH"
                '';
            };
        };

}