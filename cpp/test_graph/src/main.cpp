#include <iostream>
#include <array>

/*
#include "sbe.h"
#include <sbepp/sbepp.hpp>
#include "generated/deribit/deribit_multicast/messages/book.hpp"

std::array<char, 256> getEncoded() {
    std::array<char, 256> buf{};
    auto book = sbepp::make_view<::deribit_multicast::messages::book>(buf.data(), buf.size());
    sbepp::fill_message_header(book);

    book.changeId(10);
    book.instrumentId(15);
    return buf;
}

void dumpBuffer(const std::array<char, 256> buf) {
    auto book = sbepp::make_view<::deribit_multicast::messages::book>(buf.data(), buf.size());
    auto m = sbepp::make_const_view<::deribit_multicast::messages::book>(buf.data(), buf.size());

    std::cout << "changeid: " << m.changeId().value() << " inst id: " << m.instrumentId().value() << std::endl;
}

void runTestSbepp() {
    auto buf = getEncoded();
    dumpBuffer(buf);
}
*/

#include "generated/com_deribit_multicast/Book.h"

using namespace com::deribit::multicast;

std::array<char, 256> encodeSbe() {
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

void dumpEncoded(std::array<char, 256>& buf) {
    MessageHeader hdr = MessageHeader();

    hdr.wrap(buf.data(), 0, 0, buf.size());
    int actingVersion = hdr.version();
    int actingBlockLength = hdr.blockLength();

    Book b;
    b.wrapForDecode(buf.data(), hdr.encodedLength(), actingBlockLength, actingVersion, buf.size());

    std::cout << "changeid: " << b.changeId() << " inst id: " << b.instrumentId() << std::endl;

}

int main(int, char**){

    auto buf = encodeSbe();
    dumpEncoded(buf);
    
    std::cout << "Hello, from test_graph!\n";
}
