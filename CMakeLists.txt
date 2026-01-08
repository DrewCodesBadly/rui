cmake_minimum_required(VERSION 4.0)
include(FetchContent)

FetchContent_Declare(SFML
    GIT_REPOSITORY https://github.com/SFML/SFML.git
    GIT_TAG 3.0.2
    GIT_SHALLOW ON
    EXCLUDE_FROM_ALL
    SYSTEM)
FetchContent_MakeAvailable(SFML)


project(UIProj LANGUAGES CXX)
add_subdirectory(src)
add_subdirectory(examples/hello_world)
