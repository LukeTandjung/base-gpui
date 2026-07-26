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
      devShells = forAllSystems (pkgs: let
        # One pinned nightly for every shell: gpui uses unstable features
        # (cold_path), so stable rustc cannot compile this project at all —
        # native builds and the pre-commit cargo hooks need this toolchain
        # just as much as the wasm build does.
        nightlyToolchain = fenix.packages.${pkgs.stdenv.hostPlatform.system}.complete.withComponents [
          "cargo"
          "rustc"
          "rust-std"
          "rust-src"
          "clippy"
          "rustfmt"
          "rust-analyzer"
        ];
      in {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
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

          packages =
            [ nightlyToolchain ]
            ++ (with pkgs; [
              cargo-watch
              ast-grep
              pre-commit
              yazi
              deno
              trunk
              lld
            ]);

          RUST_SRC_PATH = "${nightlyToolchain}/lib/rustlib/src/rust/library";

          # fenix's component symlink tree breaks tool binaries' relative
          # rpath to libLLVM.dylib on macOS; dyld consults this only after
          # rpath lookup fails.
          DYLD_FALLBACK_LIBRARY_PATH = "${nightlyToolchain}/lib";
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

            echo "Rust development shell active! (fenix nightly)"
            echo "Run 'pre-commit install --hook-type pre-commit --hook-type commit-msg --hook-type pre-push' once to enable local checks."
          '';
        };

        # Toolchain for the wasm showcase: GPUI's threaded web build needs
        # build-std with atomics on top of the shared nightly toolchain.
        # site/.envrc loads it automatically via direnv.
        wasm = pkgs.mkShell {
          packages = [
            nightlyToolchain
            pkgs.trunk
            pkgs.lld
            pkgs.deno
          ];

          CARGO_UNSTABLE_BUILD_STD = "std,panic_abort";

          # fenix's component symlink tree breaks rust-lld's relative rpath
          # to libLLVM.dylib on macOS; dyld consults this only after rpath
          # lookup fails.
          DYLD_FALLBACK_LIBRARY_PATH = "${nightlyToolchain}/lib";
        };
      });
    };
}
