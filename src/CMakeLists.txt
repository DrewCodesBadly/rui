add_library(UILib lib.cpp)
target_include_directories(UILib PUBLIC ../include)

target_link_libraries(UILib PRIVATE SFML::Graphics)
