#pragma once

#include "./vector.h"

namespace GraphImpl {

class Edge {
private:
    const Vector& _source;
    const Vector& _target;

public:
    Edge(const Vector& source, const Vector& target)
        : _source(source),
          _target(target)
    {}
};

}