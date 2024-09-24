## Generate IDL Files

D:\graph>.\tools\flatc.exe --cpp -o ./cpp/generated ./proto/common.fbs ./proto/book.fbs

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

