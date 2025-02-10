export BufferedArray, count, read, read_custom, buffer_to_value

module BufferedArray

using ..Framework

struct Instance{T}
    _buffer::BufferDirect.Instance
    _start_pos::Int
    _count::Int
    Instance{T}(buffer::BufferDirect.Instance, start::Int) where {T} = new{T}(buffer, start, BufferDirect.read(buffer, Ref(start), Int64))
end
count(x::Instance)::Int = x._count
function read_array_element_at(x::Instance, pos::Int, ::Type{T}) where {T}
    pos = sizeof(Int) + x._start_pos + pos - 1
    BufferDirect.read(x._buffer, Ref(pos), T)
end
function Framework.BufferDirect.read_custom(buf::BufferDirect.Instance, pos::Ref{Int}, ::Type{Instance{T}})::Instance{T} where {T}
    # TODO: I think I need to increment the position
    Instance{T}(buf, pos[])
end

function to_value_impl(x::BufferedArray.Instance{T}, ::Type{U}) where {T,U}
    index = 1
    iter = BufferedIter.as_iter(x)
    vec = Vector{U}(undef, x._count)
    while BufferedIter.has_next(iter)
        item = BufferedIter.next(iter)
        vec[index] = buffer_to_value(item)
        index += 1
    end
    vec
end

mutable struct Writer{T}
    _buffer::BufferDirect.Instance
    _start_pos::Int
    _count::Int
    _writer_pos::Ref{Int}
    function Writer{T}(buffer::BufferDirect.Instance, writer_pos::Ref{Int}) where {T}
        _start_pos = writer_pos[]
        writer_pos[] += sizeof(Int)
        new{T}(buffer, _start_pos, 0, writer_pos)
    end
end
count(x::Writer)::Int = x._count
function create_item(x::Writer{T}) where {T}
    x._count += 1
    BufferDirect.write!(x._buffer, x._start_pos, x._count, Int)
    return Framework.create_new(x._buffer, x._writer_pos, BufferedObj.Writer{T})
end

function create_item(x::Writer{BufferedObj.Writer{T}}) where {T}
    x._count += 1
    BufferDirect.write!(x._buffer, x._start_pos, x._count, Int)
    return Framework.create_new(x._buffer, x._writer_pos, BufferedObj.Writer{T})
end


end # End Module BufferedArray

function Framework.buffer_to_value(x::BufferedArray.Instance{T}) where {T}
    BufferedArray.to_value_impl(x, T)
end


function Framework.buffer_to_value(x::BufferedArray.Instance{T}) where {T<:BufferObjT{U}} where {U}
    BufferedArray.to_value_impl(x, U)
end



