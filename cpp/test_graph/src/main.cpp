#include <iostream>
#include <array>

#include "sbeEncode.h"
#include "sbeppEncode.h"
#include "fbeEncode.h"

int main(int, char**){
    {
        std::cout << "< -------- Using SbeEncode ------ >" << std::endl;;
        auto buf = SbeEncoding::encodeBook();
        SbeEncoding::decodeBook(buf);
        std::cout << "< -------- End Using SbeEncode ------ >" << std::endl;
    }

    {
        std::cout << "< -------- Using SbeppEncode ------ >" << std::endl;;
        auto buf = SbeppEncoding::encodeBook();
        SbeppEncoding::decodeBook(buf);
        std::cout << "< -------- End Using SbeppEncode ------ >" << std::endl;
    }

    {
        std::cout << "< -------- Using FbeEncode ------ >" << std::endl;;
        auto buf = FbeEncoding::encodeBook();
        FbeEncoding::decodeBook(buf);
        std::cout << "< -------- End Using SbeEncode ------ >" << std::endl;
    }

}
