#include <iostream>

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

int main(int, char**){
    auto buf = getEncoded();
    dumpBuffer(buf);

    std::cout << "Hello, from test_graph!\n";
}
