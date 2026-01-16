add_executable(HelloWorldExample main.cpp)
target_include_directories(UILib PUBLIC ../include)
target_link_libraries(HelloWorldExample PRIVATE UILib SFML::Graphics)

add_custom_target(copyAssets COMMAND ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/assets  ${CMAKE_CURRENT_BINARY_DIR}/assets)
add_dependencies(HelloWorldExample copyAssets)
