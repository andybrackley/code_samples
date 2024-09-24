#include "processBook.h"

#include "createBook.h"
#include "../../generated/book_generated.h"

#include <iostream>

namespace Graph {
namespace FlatBufferUtils {

void FromBuffer(const std::vector<BufferT>& buf) {
    auto bookUpdate = Graph::GetBookUpdate(buf.data());

    auto instId = bookUpdate->id();
    auto exch = instId->exchange(); 

    std::cout << "InstId: " << Graph::EnumNameExchange(exch) << instId->id()->c_str() << std::endl;
}

}
}