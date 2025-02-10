# Redef of the Math module to prevent having to import the package
module MyMath
export Min

Min(a, b) = a < b ? a : b
Max(a, b) = a > b ? a : b
end # End Module Math

module ExampleSerialize

const Optional{T} = Union{T,Nothing}
Optional{T}(value::T) where {T} = Optional{T}(value)
Optional{T}() where {T} = Optional{T}(nothing)

const Scalar = Union{Char,Bool,
    Int8,Int16,Int32,Int64,
    UInt8,UInt16,UInt32,UInt64,
    Float32,Float64}

using ..MyMath

module Buffer

end ## End Module Buffer

struct BufferDirect
    _buffer::Vector{UInt8}
    _start_offset::Int
    _size::Int
    BufferDirect(size::Int) = new(Vector{UInt8}(undef, size), 1, size)
    BufferDirect(buffer::Vector{UInt8}, start_offset::Int) = new(buffer, start_offset, MyMath.Max(length(buffer) + 1 - start_offset, 0))
end
function get_absolute_pos(x::BufferDirect, pos::Int)::Int
    (x._start_offset + pos) - 1
end

function check_bounds(x::BufferDirect, pos::Int, size::Int)

end

function write_bytes!(x::BufferDirect, pos::Int, bytes::Vector{UInt8})::Int
    type_size = length(bytes)
    abs_pos = get_absolute_pos(x, pos)
    @assert pos + type_size < x._size

    # Copy bytes to the buffer at the specified position
    copyto!(x._buffer, abs_pos, bytes, 1, type_size)
    new_pos = pos + type_size
    return new_pos
end

function write!(x::BufferDirect, pos::Int, value::T)::Int where {T<:Scalar}
    bytes = reinterpret(UInt8, [value])
    write_bytes!(x, pos, Vector{UInt8}(bytes))
end

function write!(x::BufferDirect, pos::Int, value::Vector{T})::Int where {T}
    count = length(value)
    pos = write!(x, pos, count)

    for index in value
        pos = write!(x, pos, index)
    end
    return pos
end

function write!(x::BufferDirect, pos::Int, value::Vector{T})::Int where {T<:Scalar}
    count = length(value)
    pos = write!(x, pos, count)
    bytes = reinterpret(UInt8, value)
    write_bytes!(x, pos, Vector{UInt8}(bytes))
end

function read_bytes(x::BufferDirect, pos::Int, count::Int)
    abs_pos = get_absolute_pos(x, pos[])
    end_pos = abs_pos + count - 1
    size = x._size
    @assert end_pos < x._size "pos: $abs_pos + count: $count = end: $end_pos is greater than buffer size: $size"

    value_bytes = x._buffer[abs_pos:end_pos]
    value_bytes
end

function read(x::BufferDirect, pos::Ref{Int}, ::Type{T})::T where {T<:Scalar}
    count = sizeof(T)
    value_bytes = read_bytes(x, pos[], count)
    value = reinterpret(T, value_bytes)[1]
    pos[] = pos[] + count
    return value
end

function read(x::BufferDirect, pos::Ref{Int}, ::Type{Vector{T}})::Vector{T} where {T<:Scalar}
    count = read(x, pos, Int)

    num_bytes = count * sizeof(T)
    bytes = read_bytes(x, pos[], num_bytes)
    vec = reinterpret(T, bytes)
    pos[] = pos[] + num_bytes
    return vec
end

function read(x::BufferDirect, pos::Ref{Int}, ::Type{Vector{T}})::Vector{T} where {T}
    count = ExampleSerialize.read(x, pos, Int64)
    vec = Vector{T}(undef, count)

    for i in 1:count
        item = ExampleSerialize.read(x, pos, T)
        vec[i] = item
    end

    vec
end

struct BufferedArray{T}
    _buffer::BufferDirect
    _start_pos::Int
    _count::Int
    BufferedArray{T}(buffer::BufferDirect, start::Int) where {T} = new{T}(buffer, start, read(buffer, Ref(start), Int64))
end
count(x::BufferedArray)::Int = x._count
function read(x::BufferedArray, pos::Int, ::Type{T}) where {T}
    pos = sizeof(Int) + x._start_pos + pos - 1
    read(x._buffer, Ref(pos), T)
end

mutable struct BufferedIterator{T}
    _array::BufferedArray
    _count::Int
    _index::Int
    _next_buffer_pos::Int  ## NOTE: This is a relative position and will be combined with the array_start_pos to get an absolute position in the buffer
    _current::Optional{T}
    BufferedIterator{T}(array::BufferedArray{T}) where {T} = new{T}(array, count(array), 1, 1, nothing)
end

as_iter(x::BufferedArray{T}) where {T} = BufferedIterator{T}(x)
count(x::BufferedIterator{T}) where {T} = x._count
has_next(x::BufferedIterator{T}) where {T} = x._index <= count(x)
current(x::BufferedIterator{T}) where {T} = x._current

function reset!(x::BufferedIterator{T}) where {T}
    x._index = 1
    x._next_buffer_pos = 1
    x._current = nothing
end

# TODO:  I think this needs to be available for the BufferDirect instead
function get_elem_size(_::BufferedIterator{T}, _::T)::Int where {T<:Scalar}
    sizeof(T)
end

function next(x::BufferedIterator{T})::T where {T}
    if !has_next(x)
        throw(ArgumentError("No more elements"))
    end

    elem = read(x._array, x._next_buffer_pos, T)

    x._current = elem
    size = get_elem_size(x, elem)
    x._next_buffer_pos += size
    x._index += 1
    return elem
end

function to_vector(x::BufferedIterator{T})::Vector{T} where {T}
    vec = Vector{T}(undef, x._count)
    while has_next(x)
        item = next(x)
        vec[x._index-1] = item
    end

    reset!(x)
    vec
end

end  # End module ExampleSerialize

module Testing

using ..ExampleSerialize

struct TestInner
    items::Vector{Int64}
end
items(x::TestInner)::Vector{Int64} = x.items
function write!(x::ExampleSerialize.BufferDirect, pos::Int, value::TestInner)::Int
    start = pos
    slots = [0]

    pos = ExampleSerialize.write!(x, pos, slots)
    pos = ExampleSerialize.write!(x, pos, value.items)

    if (length(slots) > 0)
        bytes = pos - start
        slots[1] = bytes
        ExampleSerialize.write!(x, start, slots)
    end
    return pos
end
function read(x::ExampleSerialize.BufferDirect, pos::Ref{Int}, ::Type{TestInner})::TestInner
    slots = ExampleSerialize.read(x, pos, Vector{Int64})
    items = ExampleSerialize.read(x, pos, Vector{Int64})
    TestInner(items)
end

struct TestOuter
    items::Vector{TestInner}
end
items(x::TestOuter)::Vector{TestInner} = x.items
function write!(x::ExampleSerialize.BufferDirect, pos::Int, value::TestOuter)::Int
    start = pos
    slots = [0]

    pos = ExampleSerialize.write!(x, pos, slots)
    pos = ExampleSerialize.write!(x, pos, value.items)

    if (length(slots) > 0)
        bytes = pos - start
        slots[1] = bytes
        ExampleSerialize.write!(x, start, slots)
    end
    return pos
end
function read(x::ExampleSerialize.BufferDirect, pos::Ref{Int}, ::Type{TestOuter})::TestOuter
    slots = ExampleSerialize.read(x, pos, Vector{Int64})
    vec = ExampleSerialize.read(x, pos, Vector{TestInner})
    TestOuter(vec)
end

struct TestInner_Buffer
    _buffer::ExampleSerialize.BufferDirect
    _start_pos::Int  # TODO:  Not sure I need this as I could just splice the buffer.
    TestInner_Buffer(buffer::ExampleSerialize.BufferDirect, start_pos::Int) = new(buffer, start_pos)
end
function get_elem_size(x::ExampleSerialize.BufferedIterator{T}, elem::TestInner_Buffer)::Int where {T<:TestInner_Buffer}
    slots = ExampleSerialize.read(elem._buffer, Ref(elem._start_pos), Vector{Int64})
    size = slots[1]
    size
end
function read(x::ExampleSerialize.BufferDirect, pos::Ref{Int}, ::Type{TestInner_Buffer})::TestInner_Buffer
    # For the read we simply return the Buffer for the starting pos...
    TestInner_Buffer(x, pos[])
end
function items(x::TestInner_Buffer)::ExampleSerialize.BufferedArray{Int64}
    start_pos = Ref(x._start_pos)
    _slots = ExampleSerialize.read(x._buffer, start_pos, Vector{Int64})
    ExampleSerialize.BufferedArray{Int64}(x._buffer, start_pos[])
end
function to_obj(x::TestInner_Buffer)::TestInner
    items_raw = items(x)
    vec = ExampleSerialize.to_vector(ExampleSerialize.BufferedIterator{Int64}(items_raw))
    TestInner(vec)
end

struct TestOuter_Buffer
    _buffer::ExampleSerialize.BufferDirect
    _start_pos::Int  # TODO:  Not sure I need this as I could just splice the buffer.
    TestOuter_Buffer(buffer::ExampleSerialize.BufferDirect, start_pos::Int) = new(buffer, start_pos)
end
function get_elem_size(_::ExampleSerialize.BufferedIterator{T}, elem::TestOuter_Buffer)::Int where {T<:TestInner_Buffer}
    size = ExampleSerialize.read(elem._buffer, elem._start_pos, Int64)
    size
end
function items(x::TestOuter_Buffer)::ExampleSerialize.BufferedArray{TestInner_Buffer}
    start_pos = Ref(x._start_pos)
    _slots = ExampleSerialize.read(x._buffer, start_pos, Vector{Int64})
    ExampleSerialize.BufferedArray{TestInner_Buffer}(x._buffer, start_pos[])
end
function to_obj(x::TestOuter_Buffer)::TestOuter
    index = 1
    items_raw = ExampleSerialize.as_iter(items(x))
    inner_vec = Vector{TestInner}(undef, items_raw._count)
    while ExampleSerialize.has_next(items_raw)
        inner = ExampleSerialize.next(items_raw)
        inner_vec[index] = to_obj(inner)
        index += 1
    end

    TestOuter(inner_vec)
end

# Declare functions as part of the ExampleSerialize module.
# TODO: Not sure how to do with with my generated code yet.
Main.ExampleSerialize.write!(x::ExampleSerialize.BufferDirect, pos::Int, value::TestInner)::Int = write!(x, pos, value)
Main.ExampleSerialize.get_elem_size(x::ExampleSerialize.BufferedIterator{TestInner_Buffer}, elem::TestInner_Buffer)::Int = get_elem_size(x, elem)

Main.ExampleSerialize.write!(x::ExampleSerialize.BufferDirect, pos::Int, value::TestOuter)::Int = write!(x, pos, value)
Main.ExampleSerialize.get_elem_size(x::ExampleSerialize.BufferedIterator{TestOuter_Buffer}, elem::TestOuter_Buffer)::Int = get_elem_size(x, elem)

Main.ExampleSerialize.read(x::ExampleSerialize.BufferDirect, pos::Ref{Int}, ::Type{TestInner})::TestInner = read(x, pos, TestInner)
Main.ExampleSerialize.read(x::ExampleSerialize.BufferDirect, pos::Ref{Int}, ::Type{TestInner_Buffer})::TestInner_Buffer = read(x, pos, TestInner_Buffer)
Main.ExampleSerialize.read(x::ExampleSerialize.BufferDirect, pos::Ref{Int}, ::Type{TestOuter})::TestOuter = read(x, pos, TestOuter)

end # End Module Testing

buf = ExampleSerialize.BufferDirect(512)
ti = Testing.TestInner([1, 2, 3, 4, 5])
ExampleSerialize.write!(buf, 1, ti)

read_pos = Ref(1)
read_ti = ExampleSerialize.read(buf, read_pos, Testing.TestInner)
@assert Testing.items(ti) == Testing.items(read_ti)

tbi = Testing.TestInner_Buffer(buf, 1)
tbi_obj = Testing.to_obj(tbi)

@assert ExampleSerialize.read(buf, Ref(1), Int64) == 1 "Pos 1 should be the sizeof(slots) for TestInner == 1"
@assert ExampleSerialize.read(buf, Ref(9), Int64) == 64 "Pos 9 should be 64 which is the total size of the Serialized TestInner struct"
@assert ExampleSerialize.read(buf, Ref(17), Int64) == 5 "Pos 17 should be 5 which is the number of items in the TestInner.Items Vector"

@assert Testing.items(ti) == Testing.items(tbi_obj)

to = Testing.TestOuter(
    [Testing.TestInner([1, 2, 3, 4, 5]),
    Testing.TestInner([6, 7, 8, 9, 10])]
)

ExampleSerialize.write!(buf, 1, to)
read_to = ExampleSerialize.read(buf, Ref(1), Testing.TestOuter)

@assert ExampleSerialize.read(buf, Ref(1), Int64) == 1 "Pos 1 should be the sizeof(slots) for TestOuter == 1"
@assert ExampleSerialize.read(buf, Ref(9), Int64) == 152 "Pos 9 should be 152 which is the total size of TestOuter"

tob = Testing.TestOuter_Buffer(buf, 1)
tob_obj = Testing.to_obj(tob)
@assert string(to) == string(tob_obj)


to2 = Testing.TestOuter(
    [Testing.TestInner([1, 2, 3]),
    Testing.TestInner([4, 5, 6, 7, 8]),
    Testing.TestInner([9, 10, 11, 12])]
)

ExampleSerialize.write!(buf, 1, to2)
tob2 = Testing.TestOuter_Buffer(buf, 1)
tob_obj2 = Testing.to_obj(tob2)
println(tob_obj2)


# Timed: time = 1.49e-5, bytes = 2576, Base.GC_Deff(2576, 0, 0, 39)
@timed ExampleSerialize.write!(buf, 1, to2)

function sum_inners(buffer::ExampleSerialize.BufferDirect)
    sum = 0

    tob = Testing.TestOuter_Buffer(buffer, 1)
    tob_iter = ExampleSerialize.as_iter(Testing.items(tob))
    while ExampleSerialize.has_next(tob_iter)
        inner = ExampleSerialize.next(tob_iter)
        inner_iter = ExampleSerialize.as_iter(Testing.items(inner))
        while ExampleSerialize.has_next(inner_iter)
            v = ExampleSerialize.next(inner_iter)
            sum += v
        end
    end

    sum
end

# Timed: time = 6.0e-6, bytes = 2736, gctime = 0.0, gcstats = Base.GC_Diff(2736, 0, 0, 60, 0, 0, 0, 0, 0)
@timed sum_inners(buf)


# Timed: time = 7.8e-6, bytes = 1664, gctime = 0.0, gcstats = Base.GC_Diff(1664, 0, 0, 24, 0, 0, 0, 0, 0))
function get(buffer::ExampleSerialize.BufferDirect)
    Testing.read(buffer, Ref(1), Testing.TestOuter)
end
@timed _ = get(buf)

# time = 1.08e-5, bytes = 3424, gctime = 0.0, gcstats = Base.GC_Diff(3424, 0, 0, 70, 0, 0, 0, 0, 0))
@timed Testing.to_obj(Testing.TestOuter_Buffer(buf, 1))


using Serialization

io = IOBuffer()

#time = 0.0280623, bytes = 66480, gctime = 0.0, gcstats = Base.GC_Diff(66480, 0, 0, 1287, 1, 0, 0, 0, 0)
@timed serialize(io, to2)

seekstart(io)

#time = 0.2063755, bytes = 15579461, gctime = 0.0, gcstats = Base.GC_Diff(15579461, 1, 0, 227750, 213, 0, 0, 0, 0)
@timed deserialize(io)