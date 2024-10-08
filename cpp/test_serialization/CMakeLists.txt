cmake_minimum_required(VERSION 3.5.0)
project(test_serialization VERSION 0.1.0 LANGUAGES C CXX)

find_package(sbepp REQUIRED)

sbeppc_compile_schema(
    SCHEMA_NAME "deribit_multicast"
    TARGET_NAME deribit_mutilcast
    OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/generated/sbepp/deribit"
    SCHEMA_FILE "${CMAKE_CURRENT_SOURCE_DIR}/src/proto/deribit_multicast.xml"
)

find_package(CapnProto REQUIRED)

set(CAPNPC_OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/generated/capnproto/deribit/")
capnp_generate_cpp(capnpDeribitSources capnpDeribitHeaders ${CMAKE_CURRENT_SOURCE_DIR}/src/proto/deribit_multicast.cpnp) 

find_package(flatbuffers REQUIRED)


file (GLOB_RECURSE mainSource CONFIGURE_DEPENDS "./src/*.cpp")
file (GLOB_RECURSE fbeSources CONFIGURE_DEPENDS "./src/generated/fbeLib/*.cpp")

add_executable(test_serialization ${mainSource} ${fbeSources} ${capnpDeribitSources})

#NOTE: I think rpcrt.lib is a windows only lib so I may need to conditionally compile this.
target_link_libraries(test_serialization sbepp::sbepp deribit_mutilcast rpcrt4.lib capnproto::capnproto flatbuffers::flatbuffers)

