
include("common.jl")

function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where {T<:Scalar}
    start = offset + 1

    # Special case for Char as the sizeof(Char) == 4 but only 1 bytes
    # is written to the file
    if T == Char 
        size = 1
        last = (start + size)
        # raw = Char(UInt32(view(bytes, start:last))[1])
        ptr = pointer(bytes) + offset
        raw = unsafe_wrap(Array, ptr, size)[1]
    else
        size = sizeof(T) 
        last = (start + size) - 1 
        raw = reinterpret(T, view(bytes, start:last))[1]
    end

    value = raw |> T
    newOffset = offset + size
    return value, newOffset
end 

function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where {T<:AbstractString}
    strlen, offset = deserialize(bytes, offset, Int32)

    ptr = pointer(bytes) + offset
    raw = unsafe_wrap(Array, ptr, strlen)
    value = String(raw)
    return value, (offset + strlen)
end 


function deserialize(bytes:: Bytes, offset::Int32, ::Type{T}) where {T<:Enum}
    ptr = pointer(bytes) + offset
    value = unsafe_wrap(Array, ptr, sizeof(T))[1]  |> T
    return value, (offset + sizeof(T))
end 

function deserialize(bytes:: Bytes, offset::Int32, ::Type{Optional{T}}) where {T<:Optional}
    # Read First Byte to determine type
    type, newOffset = deserialize(bytes, offset, Char)
    if(type === Char(0)) 
        return nothing, newOffset
    end

    # union_all = Base.unwrap_unionall(T)
    # tail = Base.tuple_type_head(union_all)
    # inner_T = Base.unwrap_unionall(T).parameters[2]
    value, newOffset = deserialize(bytes, Int32(offset), T)
    return value, newOffset
end
