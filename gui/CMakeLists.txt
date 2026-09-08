cmake_minimum_required(VERSION 3.20)
project(omarchy-theme-showcase CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Quick)
qt_standard_project_setup(REQUIRES 6.5)

# Builds the Rust library and hands CMake the shared object it produces (Cargo.toml [lib] builds a
# cdylib alongside its usual rlib); nothing here touches Cargo's own build graph beyond invoking
# it. The .so carries a SONAME of libomarchy_theme.so.1, which is the name the loader will look
# for, so the link goes through a symlink under that name rather than Cargo's unversioned output.
set(RUST_PROFILE "$<IF:$<CONFIG:Debug>,debug,release>")
set(RUST_SHAREDLIB "${CMAKE_SOURCE_DIR}/../src/target/${RUST_PROFILE}/libomarchy_theme.so")
set(RUST_LIB "${CMAKE_CURRENT_BINARY_DIR}/libomarchy_theme.so.1")

add_custom_target(
    omarchy_theme_rust
    COMMAND sh -c "cargo build $<$<NOT:$<CONFIG:Debug>>:--release>"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/.."
    BYPRODUCTS "${RUST_SHAREDLIB}"
    COMMENT "Building the Rust color library"
    VERBATIM
)

add_custom_command(
    OUTPUT "${RUST_LIB}"
    COMMAND "${CMAKE_COMMAND}" -E create_symlink "${RUST_SHAREDLIB}" "${RUST_LIB}"
    DEPENDS omarchy_theme_rust
    COMMENT "Linking the color library under its SONAME"
    VERBATIM
)
add_custom_target(omarchy_theme_soname DEPENDS "${RUST_LIB}")

qt_add_executable(omarchy-theme-showcase
    src/main.cpp
    bridge/ThemeBridge.h
    bridge/ThemeBridge.cpp
)

qt_add_qml_module(omarchy-theme-showcase
    URI OmarchyShowcase
    VERSION 1.0
    QML_FILES
        ui/Main.qml
        ui/Theme.qml
        ui/SectionHeader.qml
        ui/StylesSection.qml
        ui/RoleCard.qml
        ui/TypographySection.qml
        ui/RolesSection.qml
        ui/PaletteSection.qml
)

add_dependencies(omarchy-theme-showcase omarchy_theme_soname)

target_include_directories(omarchy-theme-showcase PRIVATE bridge)
target_link_libraries(omarchy-theme-showcase PRIVATE
    Qt6::Quick
    "${RUST_LIB}"
    pthread
    dl
    m
)

target_compile_options(omarchy-theme-showcase PRIVATE
    -Wall
    -Wextra
    -Wpedantic
    -Wconversion
    -Wsign-conversion
    -Wshadow
    -Werror
)
