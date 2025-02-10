const Optional{T} = Union{Nothing,T}

const Scalar = Union{Char,Bool,
    Int8,Int16,Int32,Int64,
    UInt8,UInt16,UInt32,UInt64,
    Float32,Float64}

IsEnum(::Type{T}) where {T} = false
IsEnum(::Type{T}) where {T<:Enum} = true

IsScalar(::Type{T}) where {T} = false
IsScalar(::Type{T}) where {T<:Scalar} = true

struct Instance
end

IsScalar(Optional{Int64})
IsScalar(Instance)
IsScalar(Int32)

function _write_bytes!(x::Instance, pos::Int, bytes::Vector{UInt8})::Int
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
    bytes = reinterpret(UInt8, [value])
    return write_bytes!(x, pos, Vector{UInt8}(bytes))
end

function _write!(x::Instance, pos::Int, value::AbstractVector{T})::Int where {T}
    len = length(value)
    pos = _write!(x, pos, len)

    vec_type = eltype(T)
    if IsScalar(vec_type) || IsEnum(T)
        pos = write_fixed_size!(x, pos, value, vec_type)
        return pos
    end

    println("Write Vector TT: ", string(vec_type), IsScalar(vec_type))

    for e in value
        write!(x, pos, e, vec_type)
    end
    return pos
end

function _write!(x::Instance, pos::Int, value::Nothing)
    println("Write Nothing")
    return pos
end


function _write!(x::Instance, pos::Int, value::T) where {T<:AbstractString}
    chars = collect(value)
    return _write!(x, pos, chars)
end

function _write!(x::Instance, pos::Int, value::T) where {T}
    println("Write Type: ", string(T))
    return pos
end

function write!(x::Instance, pos::Int, value::T, ::Type{TT})::Int where {T,TT}
    if IsScalar(TT) || IsEnum(TT)
        println("write!() writing scalar for type: ", string(TT))
        return _write!(x, pos, value)
    end

    if TT isa Union
        union_types = Base.uniontypes(TT)
        vt = typeof(value)
        index = findfirst(x -> x == vt, union_types)

        println("write_union index: $index, for type: $vt, union: $union_types")
        @assert !isnothing(index) "Failed to find type: '$vt' in Union type: '$union_types'"

        pos = _write!(x, pos, index)
        return write!(x, pos, value, vt)
    end

    return _write!(x, pos, value)
end

function write_fixed_size!(x::Instance, pos::Int, value::AbstractVector{T}, ::Type{TT}) where {T,TT}
    if IsScalar(TT) || IsEnum(TT)
        bytes = reinterpret(UInt8, value)
        return _write_bytes!(x, pos, Vector{UInt8}(bytes))
    end

    for index in value
        pos = write!(x, pos, index, TT)
    end
    return pos
end




inst = Instance()

write!(inst, 1, 10, Int32)
write!(inst, 1, 10, Optional{Int64})

write!(inst, 1, [1, 2, nothing, 3, nothing, 4], Vector{Optional{Int64}})
write!(inst, 1, 10, Union{Optional{Int64}})
write!(inst, 1, "xxxxx", Optional{String})


@enum Testing begin
    val1
    val2
    val3
end

write!(inst, 1, val1, Testing)

_write!(inst, 1, "xxxx")

write!(inst, 1, inst, Instance)


# Named parameter types
function set(; param1::Int64, param2::Int64)
    println("param1: $param1, param2: $param2")
end

set(param1=10, param2=20)
