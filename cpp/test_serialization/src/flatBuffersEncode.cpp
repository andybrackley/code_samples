#include "flatBuffersEncode.h"

#include <iostream>

#include "./generated/flatbuffers/deribit_multicast_generated.h"

namespace FlatBuffersEncode {

    std::vector<uint8_t> encodeBook() {
        flatbuffers::FlatBufferBuilder builder(1024);
        auto bookName = builder.CreateString("Test from flatbuffers");

        Deribit_Multicast::BookBuilder bookBuilder(builder);

        bookBuilder.add_changed_id(10);
        bookBuilder.add_instrument_id(10);
        bookBuilder.add_name(bookName);
        
        auto offset = bookBuilder.Finish();
        Deribit_Multicast::FinishBookBuffer(builder, offset);
        auto bufferPointer = builder.GetBufferPointer();

        std::vector<uint8_t> buf;
        buf.insert(buf.end(), bufferPointer, bufferPointer + builder.GetSize());
        return buf;
    }

    void decodeBook(std::vector<uint8_t>& buf) {
        auto book = Deribit_Multicast::GetBook(buf.data());

        std::cout << "changeid: " << book->changed_id() << " inst id: " << book->instrument_id() << ":" << book->name()->c_str() << std::endl;
    }
}