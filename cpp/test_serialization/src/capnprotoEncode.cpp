#include "capnprotoEncode.h"

#include <iostream>

#include <capnp/message.h>
#include <capnp/serialize.h>

#include "./generated/capnproto/deribit/src/proto/deribit_multicast.cpnp.h"

namespace CapnProtoEncoding {

std::vector<unsigned char> encodeBook() {
    capnp::MallocMessageBuilder output;
    Book::Builder builder = output.initRoot<Book>();

    builder.setChangedId(20);
    builder.setInstrumentId(200);
    
    // Serialize the message to a byte array
    kj::Array<capnp::word> words = capnp::messageToFlatArray(output);
    kj::ArrayPtr<capnp::word> wordPtr = words.asPtr();
    kj::ArrayPtr<const kj::byte> bytePtr = wordPtr.asBytes();

    // Copy the serialized data to a std::vector
    std::vector<unsigned char> buf(bytePtr.begin(), bytePtr.end());
    return buf;
 }

void decodeBook(std::vector<unsigned char>& buf) {
    kj::ArrayPtr<unsigned char> ptr(buf.data(), buf.size());
    kj::ArrayInputStream buffer(ptr);

    capnp::InputStreamMessageReader input(buffer);
    Book::Reader book = input.getRoot<Book>();
    
    std::cout << "changeid: " << book.getChangedId() << " inst id: " << book.getInstrumentId() << std::endl;
}

}