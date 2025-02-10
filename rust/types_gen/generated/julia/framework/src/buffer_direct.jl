export BufferDirect

module BufferDirect

using ..Framework

struct Instance
    _buffer::Vector{UInt8}
    _start_offset::Int
    _size::Int
    Instance(size::Int) = new(Vector{UInt8}(undef, size), 1, size)
    Instance(buffer::Vector{UInt8}, start_offset::Int) = new(buffer, start_offset, max(length(buffer) + 1 - start_offset, 0))
end

buffer(x::Instance) = x._buffer
size(x::Instance) = x._size
start_offset(x::Instance) = x._start_offset
get_abs_pos(x::Instance, relative::Int)::Int = (x._start_offset + relative) - 1

function get_elem_size(_::BufferDirect.Instance, _::T)::Int where {T<:Scalar}
    serialized_size(T)
end

function write_custom!(x::Instance, pos::Int, value::T, ::Type{TT}) where {T<:Scalar,TT}
    type = string(TT)
    @assert false "An override of write_custom!() not found for type: $type"
end

function write_bytes!(x::Instance, pos::Int, bytes::AbstractArray{UInt8})::Int
    type_size = length(bytes)
    abs_pos = get_abs_pos(x, pos)
    buf_size = x._size
    @assert pos + type_size < buf_size "current pos: $pos, with abs_pos: $pos + type_size: $abs_pos exceeds buffer size of: $buf_size"

    # Copy bytes to the buffer at the specified position
    copyto!(x._buffer, abs_pos, bytes, 1, type_size)
    new_pos = pos + type_size
    return new_pos
end

function _write!(x::Instance, pos::Int, value::T)::Int where {T<:Scalar}
    return write_bytes!(x, pos, reinterpret(UInt8, [value]))
end

function _write!(x::Instance, pos::Int, value::AbstractVector{T})::Int where {T}
    len = length(value)
    pos = _write!(x, pos, len)
    vec_type = T

    if is_scalar(vec_type) || is_enum(T)
        pos = write_fixed_size!(x, pos, value, vec_type)
        return pos
    end

    for e in value
        pos = write!(x, pos, e, vec_type)
    end
    return pos
end

function _write!(x::Instance, pos::Int, value::Nothing)
    return pos
end

function _write!(x::Instance, pos::Int, value::T) where {T<:AbstractString}
    chars = collect(value)
    return _write!(x, pos, chars)
end

function write!(x::Instance, pos::Int, value::T, ::Type{TT})::Int where {T,TT}
    if is_scalar(TT) || is_enum(TT)
        @assert T == TT "typeof: $T does not match typeof: $TT"
        return _write!(x, pos, value)
    end

    if TT isa Union
        vt = typeof(value)
        index = type_to_index(TT, vt)
        pos = _write!(x, pos, index)
        if isnothing(value)
            # TODO: I think I might be able to Pad if ALL the types for the Union are Scalars
            return pos
        end

        return write!(x, pos, value, vt)
    end

    if TT <: AbstractVector || TT <: AbstractArray
        return _write!(x, pos, value)
    end

    if TT <: AbstractString
        return _write!(x, pos, value)
    end

    return write_custom!(x, pos, value, TT)
end

function write_fixed_size!(x::Instance, pos::Int, value::AbstractVector{T}, ::Type{TT}) where {T,TT}
    if is_scalar(TT) || is_enum(TT)
        bytes = reinterpret(UInt8, value)
        return write_bytes!(x, pos, bytes)
    end

    for index in value
        pos = write!(x, pos, index, TT)
    end
    return pos
end

function read_custom(x::Instance, pos::Ref{Int}, ::Type{T})::T where {T<:Scalar}
    type = string(TT)
    @assert false "An override of read_custom() not found for type: $type"
end


function read_bytes(x::Instance, pos::Int, count::Int)
    abs_pos = get_abs_pos(x, pos[])
    end_pos = abs_pos + count - 1
    size = x._size
    @assert end_pos < x._size "pos: $abs_pos + count: $count = end: $end_pos is greater than buffer size: $size"

    value_bytes = x._buffer[abs_pos:end_pos]
    value_bytes
end

function read(x::Instance, pos::Ref{Int}, ::Type{T})::T where {T}
    if is_scalar(T) || is_enum(T)
        count = sizeof(T)
        bytes = read_bytes(x, pos[], count)
        value = reinterpret(T, bytes)[1]
        pos[] = pos[] + count
        return value
    end

    if (T == Nothing)
        pos[] = pos[] + sizeof(UnionId)
        return nothing
    end

    if T isa Union
        type_index = read(x, pos, UnionId)
        vt = index_to_type(T, type_index)

        if vt == Nothing # Special case for Nothing
            return nothing
        end

        return read(x, pos, vt)
    end

    if T <: AbstractVector || T <: AbstractArray
        count = read(x, pos, Int)
        return read_fixed_size(x, pos, count, T)
    end

    if T <: AbstractString
        count = read(x, pos, Int)
        return String(read_fixed_size(x, pos, count, Vector{Char}))
    end

    return read_custom(x, pos, T)
end

function read_fixed_size(x::Instance, pos::Ref{Int}, count::Int, ::Type{VecT})::VecT where {VecT<:AbstractVector{T}} where {T}
    if is_scalar(T) || is_enum(T)
        num_bytes = count * sizeof(T)
        bytes = read_bytes(x, pos[], num_bytes)
        vec = reinterpret(T, bytes)
        pos[] = pos[] + num_bytes
        return vec
    end

    vec = Vector{T}(undef, count)
    for i in 1:count
        vec[i] = read(x, pos, T)
    end
    return vec
end

# TODO: Factor this and the above into one. 
function read_fixed_size(x::Instance, pos::Ref{Int}, count::Int, ::Type{ArrT})::ArrT where {ArrT<:AbstractArray{T}} where {T}
    if is_scalar(T) || is_enum(T)
        num_bytes = count * sizeof(T)
        bytes = read_bytes(x, pos[], num_bytes)
        vec = reinterpret(T, bytes)
        pos[] = pos[] + num_bytes
        return vec
    end

    vec = Vector{T}(undef, count)
    for i in 1:count
        vec[i] = read(x, pos, T)
    end
    return vec
end

export BufferDirect, read, write!, get_elem_size

end

