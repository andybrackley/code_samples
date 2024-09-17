#pragma once

#include <vector>

#include "book.h"

namespace Adhoc {

using BufferT = uint8_t;

template<typename T> 
struct Model {
    size_t size;

    int32_t id;
    uint64_t instId;

    size_t nameLength;
    const char* name;
};

using BookModel = Model<Book>;

std::vector<BufferT> serialize(const Book& b);
void deserialize(const std::vector<BufferT>& buffer);

}
