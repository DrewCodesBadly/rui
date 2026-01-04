add_executable(HelloWorldExample main.cpp)
target_include_directories(UILib PUBLIC ../include)
target_link_libraries(HelloWorldExample PRIVATE UILib)
