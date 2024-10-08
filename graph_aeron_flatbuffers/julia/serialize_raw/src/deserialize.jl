
include("common.jl")

function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where {T<:Scalar}
    ptr = pointer(bytes) + offset
    value = unsafe_wrap(Array, ptr, sizeof(T))[1]  |> T
    return value, (offset + sizeof(T))
end 

function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where {T<:AbstractString}
    strlen, offset = deserialize(bytes, offset, Int64)

    ptr = pointer(bytes) + offset
    value = unsafe_wrap(Vector{UInt8}, ptr, strlen)
    asStr = String(value)

    toread = strlen * sizeof(Char)
    return asStr, (offset + toread)
end 


function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where {T<:Enum}
    ptr = pointer(bytes) + offset
    value = unsafe_wrap(Array, ptr, sizeof(T))[1]  |> T
    return value, (offset + sizeof(T))
end 