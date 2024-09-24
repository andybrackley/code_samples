cmake_minimum_required(VERSION 3.27)
project(graph_aeron_flatbuffers VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)

find_package(aeron REQUIRED)
find_package(flatbuffers REQUIRED)

file (GLOB_RECURSE mainSource CONFIGURE_DEPENDS "./src/*.cpp")

add_executable(${PROJECT_NAME} ${mainSource} )
target_link_libraries(${PROJECT_NAME} 
PRIVATE
    flatbuffers::flatbuffers
    aeron::aeron
)