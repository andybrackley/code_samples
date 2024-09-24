#pragma once

#include <functional>

namespace GraphImpl {

class Vector
{
private:
    std::function<void()> _what;

public: 
    explicit Vector(const std::function<void()>& what)
        : _what(what)
     {}

    void execute() const {
        _what();
    }
};

}