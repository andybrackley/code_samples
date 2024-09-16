#include "sbeppEncode.h"

#include <iostream>


#include "generated/sbepp/deribit/deribit_multicast/messages/book.hpp"

namespace SbeppEncoding {

std::vector<char> encodeBook() {
    std::vector<char> buf(256);
    auto book = sbepp::make_view<::deribit_multicast::messages::book>(buf.data(), buf.size());
    sbepp::fill_message_header(book);

    book.changeId(10);
    book.instrumentId(15);
    return buf;
}

void decodeBook(std::vector<char>& buf) {
    auto book = sbepp::make_view<::deribit_multicast::messages::book>(buf.data(), buf.size());
    auto m = sbepp::make_const_view<::deribit_multicast::messages::book>(buf.data(), buf.size());

    std::cout << "changeid: " << m.changeId().value() << " inst id: " << m.instrumentId().value() << std::endl;
}
}