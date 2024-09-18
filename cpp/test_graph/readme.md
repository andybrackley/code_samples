# Test_Graph

[GitHub](https://github.com/andybrackley/code_samples/tree/main/cpp/test_graph)

## Build

$ conan profile detect
$ conan install . --output-folder=build --build=missing
$ cmake --list-presets

$ cmake --preset conan-default
$ cmake --build --preset conan-release

### Debug

$ conan install . --output-folder=build --build=missing --settings=build_type=Debug

$ cmake --build --preset conan-debug

### Clean

$ rm -rf build

## Libs

### Cap'n'Proto

[Conan](https://conan.io/center/recipes/capnproto?version=1.0.2)
[Wiki](https://capnproto.org/)
[GitHub](https://github.com/capnproto/capnproto)

### SBE

Tool downloaded from here: https://nuget.info/packages/sbe-tool/1.23.1.1

D:\code_samples\cpp\test_graph>java -Dsbe.target.language=cpp -jar .\tools\sbe-tool-all.jar src/proto/deribit_multicast.xml

[GitHub](https://github.com/real-logic/simple-binary-encoding)

### SBEPP

[SBEPP Home](https://oleksandrkvl.github.io/sbepp/1.4.0/index.html)

### FBE

D:\code_samples\cpp\test_graph>.\tools\fbec --cpp -i ./src/proto/deribit_multicast.fbe -o ./src/generated/fbeLib

[Docs](https://chronoxor.github.io/FastBinaryEncoding/)
[GitHub](https://github.com/chronoxor/FastBinaryEncoding)

### FlatBuffers

[CMake_Usage](https://github.com/ogmacorp/OgmaNeo/blob/3edd6b3e9ff044ab7a924d1871c92b8762390379/CMakeLists.txt#L152)

## Samples

[chronoxor_github](https://github.com/chronoxor/CppSerialization)

## Other Repos With Benchmarking

[ChronoXor](https://github.com/chronoxor/CppSerialization)
