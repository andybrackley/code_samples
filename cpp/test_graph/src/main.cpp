#include <iostream>
#include <array>

#include "sbeEncode.h"
#include "sbeppEncode.h"
#include "fbeEncode.h"

template<typename T, typename TId>
class TypedId {
private:
    const TId _id;

public:
    TypedId(TId id) : _id(id) {}
    TId get() const { return _id; } 

    const bool operator==(const TypedId<T, TId>& rhs) const {
        return _id == rhs._id;
    } 

    const bool operator!=(const TypedId<T, TId>& rhs) const {
        return !(this == rhs);
    } 
};

struct VectorIdTag {};
using VectorId = TypedId<VectorIdTag, int>;

template<typename BufferT> using EncoderT = BufferT(*)();
template<typename BufferT> using DecoderT = void(*)(BufferT&);

template<typename BufferT>
void run(const std::string& name, EncoderT<BufferT> encoder, DecoderT<BufferT> decoder) {
    std::cout << "<--- Start Using " << name << " ------->" << std::endl;
    auto buf = encoder();
    decoder(buf);
    std::cout << "<--- End Using " << name << " ------->" << std::endl;
}


int main(int, char**){
    run("SbeEncoding", SbeEncoding::encodeBook, &SbeEncoding::decodeBook);
    run("SbeppEncoding", SbeppEncoding::encodeBook, &SbeppEncoding::decodeBook);
    run("FbeEncoding", FbeEncoding::encodeBook, &FbeEncoding::decodeBook);

    run("SbeEncoding::SbeppDecoding", SbeEncoding::encodeBook, &SbeppEncoding::decodeBook);
    run("SbeppEncoding::SbeDecoding", SbeppEncoding::encodeBook, &SbeEncoding::decodeBook);
}
