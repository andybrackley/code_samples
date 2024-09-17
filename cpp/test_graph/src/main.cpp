#include <iostream>
#include <fstream>
#include <array>


#include "capnprotoEncode.h"
#include "sbeEncode.h"
#include "sbeppEncode.h"
#include "fbeEncode.h"

#include "adhocSerialization/book.h"
#include "adhocSerialization/bookModel.h"


const std::string getFilename(const std::string& name) {
    return "d:\\code_samples\\serialized\\" + name + ".cpp.bin";
}

template<typename BufferT> using EncoderT = BufferT(*)();
template<typename BufferT> using DecoderT = void(*)(BufferT&);

template<typename BufferT>
void run(const std::string& name, EncoderT<BufferT> encoder, DecoderT<BufferT> decoder) {
    std::cout << "<--- Start Using " << name << " ------->" << std::endl;
    auto buf = encoder();
    decoder(buf);

    std::cout << "<--- Writing to file " << name << " ------->" << std::endl;
    std::ofstream outfile(getFilename(name), std::ios::out | std::ios::binary);
    outfile.write(reinterpret_cast<const char*>(buf.data()), buf.size());
    outfile.flush();
    outfile.close();
    
    std::cout << "<--- End Using " << name << " ------->" << std::endl;
}

template<typename BufferT>
void readfile(const std::string& name, DecoderT<std::vector<BufferT>> decoder) {
    std::ifstream file(getFilename(name), std::ios::binary);

    // Stop eating new lines in binary mode
    file.unsetf(std::ios::skipws);

    // Get the file size
    file.seekg(0, std::ios::end);
    std::streampos fileSize = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<BufferT> buffer;
    buffer.reserve(fileSize);

    // Read the file data into the vector
    buffer.insert(buffer.begin(), std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>());
    file.close();

    std::cout << "Reading from File: " << name << " " << std::endl;
    decoder(buffer);
}

int main(int, char**){
    size_t size = 5;
    int32_t id = 15;
    uint64_t instId = 700;
    char* name = "Test Name";


    Adhoc::Book b = Adhoc::Book { 1, 50, 1000 };
    b.name = "Testing";

    const std::vector<Adhoc::BufferT> buf = Adhoc::serialize(b);
    
    std::ofstream outfile(getFilename("Adhoc"), std::ios::out | std::ios::binary);
    outfile.write(reinterpret_cast<const char*>(buf.data()), buf.size());
    outfile.flush();
    outfile.close();

    
    Adhoc::deserialize(buf);

    printf("Sizeof Book %zu", sizeof(b));

    // run("SbeEncoding", SbeEncoding::encodeBook, SbeEncoding::decodeBook);
    // run("SbeppEncoding", SbeppEncoding::encodeBook, SbeppEncoding::decodeBook);
    // run("FbeEncoding", FbeEncoding::encodeBook, FbeEncoding::decodeBook);
    // run("CapNProtoEncoding", CapnProtoEncoding::encodeBook, CapnProtoEncoding::decodeBook);

    // run("SbeEncoding.SbeppDecoding", SbeEncoding::encodeBook, SbeppEncoding::decodeBook);
    // run("SbeppEncoding.SbeDecoding", SbeppEncoding::encodeBook, SbeEncoding::decodeBook);

    // readfile("SbeEncoding", SbeEncoding::decodeBook);
    // readfile("CapnProtoEncoding", CapnProtoEncoding::decodeBook);
    // readfile("FbeEncoding", FbeEncoding::decodeBook);
}
