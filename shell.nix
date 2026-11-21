with import <nixpkgs> { };

mkShell {
  pname = "raylib-rs-dev";

  buildInputs = [
    wayland
    wayland-protocols

    gcc
    gcc-unwrapped
    gnumake
    clang
    cmake
    kdePackages.extra-cmake-modules

    mesa
    libglvnd
    glfw
    xwayland
    libx11
    libxcb
    libxkbcommon

    libGL
    libxrandr
    libxinerama
    libxcursor
    libxi

    llvmPackages.bintools
  ];

  LD_LIBRARY_PATH =
    with pkgs;
    lib.makeLibraryPath [
      libGL
      libxrandr
      libxinerama
      libxcursor
      libxi
    ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

  RUSTFLAGS = "-C linker=${pkgs.clang}/bin/clang";
  # lgcc_s isn't working for some reason
  #RUSTFLAGS = "-C linker=${pkgs.llvmPackages.bintools}/bin/ld";

  shellHook = ''
    clear
    export GDK_BACKEND=x11
    export QT_QPA_PLATFORM=xcb
    export SDL_VIDEO_DRIVER=x11
  '';
}
