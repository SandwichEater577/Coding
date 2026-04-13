cmake_minimum_required(VERSION 3.10)
project(MatrixDE_Core)

set(CMAKE_CXX_STANDARD 17)

# Find X11 library
find_package(X11 REQUIRED)

include_directories(${X11_INCLUDE_DIR})

add_executable(MatrixCore main.cpp)

target_link_libraries(MatrixCore ${X11_LIBRARIES} pthread)
