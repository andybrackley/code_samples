export Optional, Scalar, UnionId, FixedSize, BufferObjT, serialized_size, calc_padding, tg_memcmp, buffer_to_value, is_enum, is_scalar, is_fixed_size, type_to_index, index_to_type

const Optional{T} = Union{Nothing,T}

# Used for Serialization of Unions to identify whether it was Some or Nothing
const UnionId = UInt8

const Scalar = Union{Char,Bool,
    Int8,Int16,Int32,Int64,
    UInt8,UInt16,UInt32,UInt64,
    Float32,Float64}

abstract type BufferObjT{T} end

@inline is_enum(::Type{T}) where {T} = false
@inline is_enum(::Type{T}) where {T<:Enum} = true

@inline is_scalar(::Type{T}) where {T} = false
@inline is_scalar(::Type{T}) where {T<:Scalar} = true

@inline function serialized_size(::Type{T})::UInt64 where {T<:Scalar}
    sizeof(T)
end

@inline function buffer_to_value(x::T) where {T<:Scalar}
    return x
end

# TODO: 
#    I think this method is quite slow.
function type_to_index(::Type{T}, ::Type{TT}) where {T,TT}
    @assert T isa Union "Type: $T is not a Union type"
    all = Base.uniontypes(T)
    index::Optional{UnionId} = findfirst(x -> x == TT, all)

    @assert !isnothing(index) "Failed to find type: '$TT' in Union type: '$all'"
    return index
end

function index_to_type(::Type{T}, index::UnionId) where {T}
    @assert T isa Union "Type: $T is not a Union type"
    all = Base.uniontypes(T)
    @assert index <= length(all) "Type index: $type_index is greater than the number of types in the Union: $all"
    return all[index]
end

# See FlatBuffers alignments....
#   flatbuffers\include\flatbuffers\flatbuffer_builder.h
# 
# template<template<typename> class OffsetT = Offset, typename LenT = uint32_t>
# void StartVector(size_t len, size_t elemsize, size_t alignment) {
#   NotNested();
#   nested = true;
#   // Align to the Length type of the vector (either 32-bit or 64-bit), so
#   // that the length of the buffer can be added without padding.
#   PreAlign<LenT>(len * elemsize);
#   PreAlign(len * elemsize, alignment);  // Just in case elemsize > uoffset_t.
# }
#
# // Aligns such that when "len" bytes are written, an object can be written
# // after it (forward in the buffer) with "alignment" without padding.
# void PreAlign(size_t len, size_t alignment) {
#   if (len == 0) return;
#   TrackMinAlign(alignment);
#   buf_.fill(PaddingBytes(GetSize() + len, alignment));
# }


function calc_padding(pos::UInt64)
    println("In CalcPadding")

    r = pos % 8
    to_pad = r == 0 ? 0 : 8 - r
    return to_pad
end

# Ripped from the cmem.jl file defined in the Julia language
function tg_memcmp(a::Ptr, b::Ptr, n::Integer)::Bool
    ccall(:memcmp, Cint, (Ptr{Cvoid}, Ptr{Cvoid}, Csize_t), a, b, n % Csize_t) % Int == 0
end
