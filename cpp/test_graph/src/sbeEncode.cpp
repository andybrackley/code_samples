#include "sbeEncode.h"

#include <iostream>

#include "generated/sbeLib/com_deribit_multicast/Book.h"

using namespace com::deribit::multicast;

namespace SbeEncoding {

std::array<char, 256> encodeBook() {
    std::array<char, 256> buf{};

    MessageHeader hdr = MessageHeader();
    hdr.wrap(buf.data(), 0, 0, buf.size())
        .blockLength(Book::sbeBlockLength())
        .templateId(Book::sbeTemplateId())
        .schemaId(Book::sbeSchemaId())
        .version(Book::sbeSchemaVersion());

    Book b;
    b.wrapForEncode(buf.data(), hdr.encodedLength(), buf.size() - hdr.encodedLength())
        .changeId(1)
        .instrumentId(5);

    return buf;
 }

void decodeBook(std::array<char, 256>& buf) {
    MessageHeader hdr = MessageHeader();

    hdr.wrap(buf.data(), 0, 0, buf.size());
    int actingVersion = hdr.version();
    int actingBlockLength = hdr.blockLength();

    Book b;
    b.wrapForDecode(buf.data(), hdr.encodedLength(), actingBlockLength, actingVersion, buf.size());

    std::cout << "changeid: " << b.changeId() << " inst id: " << b.instrumentId() << std::endl;

}

}