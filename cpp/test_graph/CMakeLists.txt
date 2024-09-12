cmake_minimum_required(VERSION 3.5.0)
project(test_graph VERSION 0.1.0 LANGUAGES C CXX)

find_package(sbepp REQUIRED)

sbeppc_compile_schema(
    SCHEMA_NAME "deribit_multicast"
    TARGET_NAME deribit_mutilcast
    OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/generated/sbepp/deribit"
    SCHEMA_FILE "${CMAKE_CURRENT_SOURCE_DIR}/src/proto/deribit_multicast.xml"
)

add_executable(test_graph ./src/main.cpp ./src/sbeEncode.h ./src/sbeEncode.cpp ./src/sbeppEncode.h ./src/sbeppEncode.cpp)

target_link_libraries(test_graph sbepp::sbepp deribit_mutilcast)

target_link_libraries(test_graph sbepp::sbepp)