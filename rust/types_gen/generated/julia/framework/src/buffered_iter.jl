export BufferedIter

module BufferedIter

using ..Framework

mutable struct Instance{T}
    _array::BufferedArray.Instance
    _count::Int
    _index::Int
    _next_buffer_pos::Int  ## NOTE: This is a relative position and will be combined with the array_start_pos to get an absolute position in the buffer
    _current::Optional{T}
    Instance{T}(array::BufferedArray.Instance{T}) where {T} = new{T}(array, BufferedArray.count(array), 1, 1, nothing)
end

as_iter(x::BufferedArray.Instance{T}) where {T} = Instance{T}(x)
count(x::Instance{T}) where {T} = x._count
has_next(x::Instance{T}) where {T} = x._index <= count(x)
current(x::Instance{T}) where {T} = x._current

function reset!(x::Instance{T}) where {T}
    x._index = 1
    x._next_buffer_pos = 1
    x._current = nothing
end

function next(x::Instance{T})::T where {T}
    if !has_next(x)
        throw(ArgumentError("No more elements"))
    end

    elem = BufferedArray.read_array_element_at(x._array, x._next_buffer_pos, T)

    x._current = elem
    size = BufferDirect.get_elem_size(x._array._buffer, elem)
    x._next_buffer_pos += size
    x._index += 1
    return elem
end

function to_vector(x::Instance{T})::Vector{T} where {T}
    vec = Vector{T}(undef, x._count)
    while has_next(x)
        item = next(x)
        vec[x._index-1] = item
    end

    reset!(x)
    vec
end

export next, has_next, reset!, to_vector, count, as_iter, current, as_iter

end