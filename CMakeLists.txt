cmake_minimum_required(VERSION 3.31)
project(led-strip-display C)

include(FetchContent)

option(SDL2 "Use SDL2 backend" OFF)
option(SDL3 "Use SDL3 backend" OFF)
option(BUILD_EXAMPLE "Build example binary" OFF)

if (NOT SDL2 AND NOT SDL3)
    message(FATAL_ERROR "No SDL backend option is set!")
elseif(SDL2 AND SDL3)
    message(FATAL_ERROR "Can't enable both SDL2 and SDL3 backend!")
endif()

# =========================
# led-strip-display library
# =========================

find_package(Corrosion QUIET) # Get system install
if (NOT Corrosion_FOUND)
    # Fallback to local fetch
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.6.1
    )
    FetchContent_MakeAvailable(Corrosion)
endif()

if (SDL2)
    set(BACKEND "sdl2")
    find_package(SDL2 REQUIRED)
elseif(SDL3)
    set(BACKEND "sdl3")
    find_package(SDL3 REQUIRED)
endif()

corrosion_import_crate(
    MANIFEST_PATH Cargo.toml
    FEATURES ${BACKEND}
    CRATE_TYPES staticlib cdylib
)

target_sources(led_strip_display INTERFACE
        FILE_SET HEADERS
        BASE_DIRS include
        FILES led-strip-display.h
)

if (SDL2)
    target_link_libraries(led_strip_display INTERFACE SDL2::SDL2)
elseif(SDL3)
    target_link_libraries(led_strip_display INTERFACE SDL3::SDL3)
endif()

# =======
# Example
# =======

if (BUILD_EXAMPLE)
    if (SDL2)
        set(EXAMPLE_NAME "sdl2-c-display")
        set(EXAMPLE_SRC "src/bin/sdl2-c-display.c")
    elseif(SDL3)
        set(EXAMPLE_NAME "sdl3-c-display")
        set(EXAMPLE_SRC "src/bin/sdl3-c-display.c")
    endif()

    add_executable(
        ${EXAMPLE_NAME}
        ${EXAMPLE_SRC}
    )
    target_link_libraries(${EXAMPLE_NAME} led_strip_display)
endif()
