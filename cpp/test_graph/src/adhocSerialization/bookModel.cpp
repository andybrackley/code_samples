#include <iostream>

#include "bookModel.h"

namespace Adhoc {

    // std::move
    // std::shared_ptr
    // std::weak_ptr
    // std::initializer_list

    // Boost -- https://www.boost.org/
    // Boost::Contract  / static_assert <cassert>
    // Boost::outcome or std::experimental::expected
    // https://en.cppreference.com/w/cpp/compiler_support
    // https://cppcon2019.sched.com/event/SiVW - Defragmenting C++ ( Herb Sutter )
    // https://www.youtube.com/watch?v=vrfYLlR8X8k&ab_channel=NOKIATechnologyCenterWroc%C5%82aw - Writing Fast Code 1 ( Andrei Alexandrescu )

    // Exception handling:
    //     Copy + Swap
    /*
        void func(std::string& str) {
            auto tmp = std::string{str};  // Copy
            tmp += f1();                  // Mutate copy, may throw
            tmp += f2();                  // Mutate copy, may throw
            std::swap(tmp, str);          // Swap, never throws
        }
    */
   // Lambdas
   /*
        // Look for numbers which is larger than three 
        auto is_above_3 = [](int v) { return v > 3; }; 
        auto num_above_3 = std::count_if(v.begin(), v.end(), is_above_3);

        // Capture a variable ( closure )

        // By Value
        auto is_above = [x](int i) { return i > x; };

        // By Reference
        auto is_above = [&x](int i) { return i > x; };

        // Hard-code values:
        auto some_func = [numbers = std::list<int>{4,2}]() {
          for (auto i : numbers)
            std::cout << i;
        };
        some_func();

   */

   // 
   //  https://github.com/google/flatbuffers/blob/master/benchmarks/cpp/raw/raw_bench.cpp
   // 
const size_t headerSize = sizeof(size_t);

const size_t serialize_sizeT(const BufferT* buffer, size_t& offset) {
    const size_t value = buffer[offset += sizeof(size_t)];
    return value;
}

std::vector<BufferT> serialize(const Book& book) {
    const char* asBytes = reinterpret_cast<const char*>(&book);

    std::vector<BufferT> buffer; 

    size_t nameLength = std::strlen(book.name) + 1; // +1 for the null terminator
    buffer.resize(sizeof(book.id) + sizeof(book.instrumentId) + sizeof(nameLength) + nameLength);
    size_t offset = 0;

    std::memcpy(buffer.data() + offset, &book.id, sizeof(book.id));
    offset += sizeof(book.id);

    std::memcpy(buffer.data() + offset, &book.instrumentId, sizeof(book.instrumentId));
    offset += sizeof(book.instrumentId);

    std::memcpy(buffer.data() + offset, &nameLength, sizeof(nameLength));
    offset += sizeof(nameLength);

    std::memcpy(buffer.data() + offset, book.name, nameLength);
    return buffer;
}

void deserialize_impl(const std::vector<uint8_t>& buffer, size_t& size, int32_t& id, uint32_t& instId, char*& name) {
    size_t offset = 0;

    // std::memcpy(&size, buffer.data() + offset, sizeof(size));
    // offset += sizeof(size);

    std::memcpy(&id, buffer.data() + offset, sizeof(id));
    offset += sizeof(id);

    std::memcpy(&instId, buffer.data() + offset, sizeof(instId));
    offset += sizeof(instId);

    size_t nameLength;
    std::memcpy(&nameLength, buffer.data() + offset, sizeof(nameLength));
    offset += sizeof(nameLength);

    name = new char[nameLength];
    std::memcpy(name, buffer.data() + offset, nameLength);
}

void deserialize_impl2(const std::vector<uint8_t>& buffer, size_t& size, int32_t& id, uint32_t& instId, const char*& name) {
    const uint8_t* data = buffer.data();
    size_t offset = 0;

    // size = *reinterpret_cast<const size_t*>(data + offset);
    // offset += sizeof(size);

    id = *reinterpret_cast<const int32_t*>(data + offset);
    offset += sizeof(id);

    instId = *reinterpret_cast<const uint64_t*>(data + offset);
    offset += sizeof(instId);

    size_t nameLength = *reinterpret_cast<const size_t*>(data + offset);
    offset += sizeof(nameLength);

    // name = new char[nameLength];
    // std::memcpy(name, data + offset, nameLength);
    name = reinterpret_cast<const char*>(data + offset);
}

void deserialize(const std::vector<uint8_t>& buffer) {
    const uint8_t* data = buffer.data();
    size_t offset = 0;

    Book b;

    deserialize_impl2(buffer, offset, b.id, b.instrumentId, b.name);

    std::cout << b.id << ":" << b.instrumentId << ":" << b.name << std::endl;
}

}
