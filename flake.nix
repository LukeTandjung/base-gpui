{
  description = "A pure Nix environment for your GPUI Project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, fenix, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      formatter = forAllSystems (pkgs: pkgs.alejandra);
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            pkg-config
          ];

          buildInputs =
            with pkgs;
            [
              fontconfig
              freetype
              libxkbcommon
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
              wayland
              libxcb
              libx11
              vulkan-loader
              mesa
              libglvnd
            ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
            (with pkgs; [
              fontconfig
              freetype
              libxkbcommon
            ])
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux (
              with pkgs;
              [
                wayland
                libxcb
                libx11
                vulkan-loader
                mesa
                libglvnd
              ]
            )
          );

          packages = with pkgs; [
            rustc
            clippy
            rust-analyzer
            cargo-watch
            ast-grep
            pre-commit
            yazi
            rustfmt
            deno
            trunk
            lld
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          SDKROOT = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
          BINDGEN_EXTRA_CLANG_ARGS = "-isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk -F/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks";

          shellHook = ''
            mkdir -p /tmp/nix-xcode-shims
            ln -sf /usr/bin/xcrun /tmp/nix-xcode-shims/xcrun
            export PATH="/tmp/nix-xcode-shims:/Applications/Xcode.app/Contents/Developer/usr/bin:$PATH"

            if [ -d /run/opengl-driver/lib ]; then
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/run/opengl-driver/lib"
            fi

            echo "Rust development shell active! (rustc ${pkgs.rustc.version})"
            echo "Run 'pre-commit install --hook-type pre-commit --hook-type pre-push' once to enable local checks."
          '';
        };

        # Toolchain for the wasm showcase: GPUI's threaded web build needs
        # nightly (build-std with atomics), so this shell swaps in a fenix
        # nightly toolchain. Use `nix develop .#wasm -c trunk build` from
        # showcase/.
        wasm =
          let
            toolchain = fenix.packages.${pkgs.stdenv.hostPlatform.system}.complete.withComponents [
              "cargo"
              "rustc"
              "rust-std"
              "rust-src"
            ];
          in
          pkgs.mkShell {
            packages = [
              toolchain
              pkgs.trunk
              pkgs.lld
              pkgs.deno
            ];

            CARGO_UNSTABLE_BUILD_STD = "std,panic_abort";

            # fenix's component symlink tree breaks rust-lld's relative rpath
            # to libLLVM.dylib on macOS; dyld consults this only after rpath
            # lookup fails.
            DYLD_FALLBACK_LIBRARY_PATH = "${toolchain}/lib";
          };
      });
    };
}
