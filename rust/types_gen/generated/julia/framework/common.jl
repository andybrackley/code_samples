const Optional{T} = Union{Nothing,T}

const Scalar = Union{Char,Bool,
    Int8,Int16,Int32,Int64,
    UInt8,UInt16,UInt32,UInt64,
    Float32,Float64}

const Bytes = AbstractVector{UInt8}

function calc_padding(pos::UInt64)
    r = pos % 8
    to_pad = r == 0 ? 0 : 8 - r
    return to_pad
end
