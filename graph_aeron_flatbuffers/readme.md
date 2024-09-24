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



## Aeron Media Driver

NOTE: We are using 1.41.6
Run the media driver as follows:

D:\code_samples\cpp\graph_aeron_flatbuffers\aeronMediaDriver>java -cp ./aeron-all-1.41.6.jar io.aeron.driver.MediaDriver

Stats:
D:\code_samples\cpp\graph_aeron_flatbuffers\aeronMediaDriver>java -cp aeron-all-1.41.6.jar io.aeron.samples.AeronStat

[Download](https://central.sonatype.com/artifact/io.aeron/aeron-all/1.46.2/versions)

