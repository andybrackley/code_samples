# TODO: I don't think this should be relying on anything outside this module
include("../../src/messages/common.jl")

const Scalar = Union{Char, Bool,
Int8, Int16, Int32, Int64,
UInt8, UInt16, UInt32, UInt64,
Float32, Float64}

const IdTypes = Union{Timestamp, Level}

const Bytes = AbstractVector{UInt8}
