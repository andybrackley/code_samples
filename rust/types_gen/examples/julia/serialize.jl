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

function serialize_bytes(buffer::Vector{UInt8}, pos::Int, bytes::Vector{UInt8})::Int
    type_size = length(bytes)
    # Copy bytes to the buffer at the specified position 
    for i in 1:type_size
        buffer[pos+i-1] = bytes[i]
    end
    new_pos = pos + type_size
    return new_pos
end

function serialize(buffer::Vector{UInt8}, pos::Int, value::T)::Int where {T<:Scalar}
    bytes = reinterpret(UInt8, [value])
    return serialize_bytes(buffer, pos, Vector{UInt8}(bytes))
end

function serialize(buffer::Vector{UInt8}, pos::Int, value::Vector{T})::Int where {T}
    vec_len = length(value)
    pos = serialize(buffer, pos, Int64(vec_len))

    pos += Int(calc_padding(UInt64(pos)))
    for i in 1:vec_len
        pos = serialize(buffer, pos, value[i])
    end
    return pos
end


buffer = Vector{UInt8}(undef, 128)
pos = 1
pos = serialize(buffer, pos, UInt64(1234))

function deserialize_bytes(buffer::Vector{UInt8}, pos::Int, size::Int)::Vector{UInt8}
    bytes = buffer[pos:pos+size-1]
    return bytes
end

function deserialize(buffer::Vector{UInt8}, pos::Ref{Int}, ::Type{T})::T where {T<:Scalar}
    size = sizeof(T)
    bytes = deserialize_bytes(buffer, pos[], size)
    value_array = reinterpret(T, bytes)
    pos[] += size
    return value_array[1]
end

function deserialize_vec(buffer::Vector{UInt8}, pos::Ref{Int}, ::Type{T})::Vector{T} where {T}
    # TODO: Naive implementation
    vec_len = deserialize(buffer, pos, Int64)
    pos[] += Int(calc_padding(UInt64(pos[])))

    result = Vector{T}(undef, vec_len)
    for i in 1:vec_len
        result[i] = deserialize(buffer, pos, T)
    end

    result
end


read_pos = Ref(1)
result = deserialize(buffer, read_pos, UInt64)[1]
println(result)
println(read_pos)


pos = 1
pos = serialize(buffer, pos, Vector{UInt64}([1, 2, 3, 4, 5]))

read_pos = Ref(1)
result = deserialize_vec(buffer, read_pos, UInt64)
println(result)
println(read_pos)


mutable struct Test
    _id::UInt64
    _name::String

    # Test(
    #     id::UInt64,
    #     name::String,
    # ) = new(
    #     id,
    #     name,
    # )
end

function create()::Test
    Test(
        _id=1234,
        _name="test",
    )

end