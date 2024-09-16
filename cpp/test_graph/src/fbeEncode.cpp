#include "fbeEncode.h"

#include <iostream>

#include "./generated/fbeLib/deribit_multicast.h"
#include "./generated/fbeLib/deribit_multicast_models.h"


namespace FbeEncoding {

std::vector<uint8_t> encodeBook() {
    deribit_multicast::Book book(1, 50, 1000);
    
    FBE::deribit_multicast::BookModel writer;
    size_t size = writer.serialize(book);

    uint8_t* b = writer.buffer().data();

    std::vector<uint8_t> buf(&b[0], &b[size]);
    return buf;
 }

void decodeBook(std::vector<uint8_t>& buf) {

    FBE::deribit_multicast::BookModel reader;
    reader.attach(buf.data(), buf.size(), 0);

    deribit_multicast::Book book;
    reader.deserialize(book);
    reader.reset();

    std::cout << "changeid: " << book.changedId << " inst id: " << book.instrumentId << std::endl;
}

}