# Test_Graph

## TODO

- Setup Google Benchmark
- Setup Aeron
- Setup proper messages ( Start with flatbuffers )
- Send/Receive multiple messages ( benchmarked )

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

Requirements

- Support for C++, Julia and Rust
- Small generated binary format
- Fast encoding/decoding
- Minimal allocations ( Possibly direct serialization from Shared Memory )
- Support for Array/String and Union types

### Cap'n'Proto

[Conan](https://conan.io/center/recipes/capnproto?version=1.0.2)
[Wiki](https://capnproto.org/)
[GitHub](https://github.com/capnproto/capnproto)

#### Capnproto Pros

    Multiple Language support

#### Capnproto Cons

    Hard to use API
    Not easy to pass decoded object around, i.e. Book::Reader

### SBE

Tool downloaded from here: https://nuget.info/packages/sbe-tool/1.23.1.1

D:\code_samples\cpp\test_graph>java -Dsbe.target.language=cpp -jar .\tools\sbe-tool-all.jar src/proto/deribit_multicast.xml

#### SBE Pros

   Written by same people as Aeron
   Built for FIX

#### SBE Cons

   Crappy IDL format ( FIX Format )

[GitHub](https://github.com/real-logic/simple-binary-encoding)

### SBEPP

SBEPP is just a convenience wrapper around SBE to make the usage slightly nicer in C++

[SBEPP Home](https://oleksandrkvl.github.io/sbepp/1.4.0/index.html)

### FBE

D:\code_samples\cpp\test_graph>.\tools\fbec --cpp -i ./src/proto/deribit_multicast.fbe -o ./src/generated/fbeLib

[Docs](https://chronoxor.github.io/FastBinaryEncoding/)
[GitHub](https://github.com/chronoxor/FastBinaryEncoding)

### FlatBuffers

See also:
[FlexBuffers](https://github.com/google/flatbuffers/blob/master/rust/flexbuffers/README.md)

[GitHub-Raw Structs](https://github.com/google/flatbuffers/blob/master/benchmarks/cpp/raw/raw_bench.cpp)

[CMake_Usage](https://github.com/ogmacorp/OgmaNeo/blob/3edd6b3e9ff044ab7a924d1871c92b8762390379/CMakeLists.txt#L152)

[Julia](https://flatbuffers.juliadata.org/stable/)

#### FlatBuffers Prod

- Multiple language support ( cpp, rust, julia )
- Easy to use API
- Readable IDL
- Support for strings/arrays and unions
- Support for optional fields in case of versioning ( thinking of message replay for testing )

### Blobify

[GitHub](https://github.com/neobrain/blobify)
[YouTube](https://www.youtube.com/watch?v=o3j6hfXDCVc&ab_channel=CppCon)
[Nintendo3DS YouTube](https://www.youtube.com/watch?v=67OCoOLVuK8&ab_channel=CppCon)

References:
[Magic_Get - GitHub](https://github.com/apolukhin/magic_get)

## Samples

[chronoxor_github](https://github.com/chronoxor/CppSerialization)

## Other Repos With Benchmarking

[ChronoXor](https://github.com/chronoxor/CppSerialization)

### Other References

    // std::move
    // std::shared_ptr
    // std::weak_ptr
    // std::initializer_list

    // Boost -- https://www.boost.org/
    // Boost::Contract  / static_assert <cassert>
    // Boost::outcome or std::experimental::expected

    // https://en.cppreference.com/w/cpp/compiler_support
    // https://cppcon2019.sched.com/event/SiVW - Defragmenting C++ ( Herb Sutter )
    // https://www.youtube.com/watch?v=vrfYLlR8X8k&ab_channel=NOKIATechnologyCenterWroc%C5%82aw - Writing Fast Code 1 ( Andrei Alexandrescu )
    // https://www.aristeia.com/Fastware/ - Fast Code ( Scott Meyers )
    // https://www.aristeia.com/Fastware/Fastware%20Introduction%202008-12-15.pdf - Fast Code ( Scott Meyers )

    // Exception handling:
    //     Copy + Swap
    /*
        void func(std::string& str) {
            auto tmp = std::string{str};  // Copy
            tmp += f1();                  // Mutate copy, may throw
            tmp += f2();                  // Mutate copy, may throw
            std::swap(tmp, str);          // Swap, never throws
        }
    */
   // Lambdas
   /*
        // Look for numbers which is larger than three 
        auto is_above_3 = [](int v) { return v > 3; }; 
        auto num_above_3 = std::count_if(v.begin(), v.end(), is_above_3);

        // Capture a variable ( closure )

        // By Value
        auto is_above = [x](int i) { return i > x; };

        // By Reference
        auto is_above = [&x](int i) { return i > x; };

        // Hard-code values:
        auto some_func = [numbers = std::list<int>{4,2}]() {
          for (auto i : numbers)
            std::cout << i;
        };
        some_func();

   */

   //
   //  https://github.com/google/flatbuffers/blob/master/benchmarks/cpp/raw/raw_bench.cpp
   //