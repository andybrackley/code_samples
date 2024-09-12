cmake_minimum_required(VERSION 3.5.0)
project(test_graph VERSION 0.1.0 LANGUAGES C CXX)

find_package(sbepp REQUIRED)

sbeppc_compile_schema(
    SCHEMA_NAME "deribit_multicast"
    TARGET_NAME deribit_mutilcast
    OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/generated/sbepp/deribit"
    SCHEMA_FILE "${CMAKE_CURRENT_SOURCE_DIR}/src/proto/deribit_multicast.xml"
)

file (GLOB_RECURSE mainSource CONFIGURE_DEPENDS "./src/*.cpp")
file (GLOB_RECURSE fbeSources CONFIGURE_DEPENDS "./src/generated/fbeLib/*.cpp")

add_executable(test_graph ${mainSource} ${fbeSources})

#NOTE: I think rpcrt.lib is a windows only lib so I may need to conditionally compile this.
target_link_libraries(test_graph sbepp::sbepp deribit_mutilcast rpcrt4.lib)
