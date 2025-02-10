export BufferedObj

function reset_write_pos() end

function writer_for()
end

function set()
end

function create_new(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{T}) where {T}
    return T()
end

module BufferedObj

using ..Framework

struct Reader{T} <: BufferObjT{T}
    _buffer::BufferDirect.Instance
    _start_pos::Int
    Reader{T}(buf::BufferDirect.Instance, pos::Int) where {T} = new{T}(buf, pos)
end

mutable struct Writer{T} <: BufferObjT{T}
    _buffer::BufferDirect.Instance
    _start_pos::Int
    _write_pos::Ref{Int}
    Writer{T}(buf::BufferDirect.Instance, pos::Ref{Int}) where {T} = new{T}(buf, pos[], pos)
end

function reset_write_pos(x::Writer)
    x._write_pos[] = x._start_pos
end

function create_new(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{Writer{T}}) where {T}
    Writer{T}(buf, pos)
end

function set(x::Writer{T}, val::T) where {T}
    BufferDirect.write!(x._buffer, x._write_pos[], val, T)
end

mutable struct UnionWriter{T} <: BufferObjT{T}
    _buffer::BufferDirect.Instance
    _start_pos::Int
    _write_pos::Ref{Int}
    UnionWriter{T}(buf::BufferDirect.Instance, pos::Ref{Int}) where {T} = new{T}(buf, pos[], pos)
end

@inline function get_elem_size(buf::BufferDirect.Instance, elem::BufferObjT{T}) where {T}
    is_fixed_size(T) ? serialized_size(T) : BufferDirect.read(elem._buffer, Ref(elem._start_pos), Int64)
end

function read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{BufferObjT{T}}) where {T}
    elem = Instance{T}(buf, pos[])
    pos[] += get_elem_size(buf, elem)
    elem
end

end

Framework.set(x::BufferedObj.Writer{T}, val::T) where {T} = BufferedObj.set(x, val)
Framework.reset_write_pos(x::BufferedObj.Writer{T}) where {T} = BufferedObj.reset_write_pos(x)
Framework.create_new(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{BufferedObj.Writer{T}}) where {T} = BufferedObj.create_new(buf, pos, BufferedObj.Writer{T})

BufferDirect.get_elem_size(buf::BufferDirect.Instance, elem::BufferObjT{T}) where {T} = BufferedObj.get_elem_size(buf, elem)
BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{BufferObjT{T}}) where {T} = BufferedObj.read_custom(buf, pos, BufferedObj.Instance{T})

function Framework.writer_for(x::BufferedObj.UnionWriter{T}, ::Type{U}) where {T,U}
    type_index = type_to_index(T, U)
    x._write_pos[] = BufferDirect.write!(x._buffer, x._write_pos[], type_index, UInt8)
    if U == Nothing
        return Nothing
    end

    return BufferedObj.Writer{U}(x._buffer, x._write_pos)
end

function Framework.writer_for(x::BufferedObj.UnionWriter{T}, ::Type{U}) where {T,U<:Union{BufferedObj.Writer,Nothing}}
    type_index = type_to_index(T, U)
    x._write_pos[] = BufferDirect.write!(x._buffer, x._write_pos[], type_index, UInt8)
    if U == Nothing
        return Nothing
    end

    return U(x._buffer, x._write_pos)
end