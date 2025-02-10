# NOTE: If you re-run you'll need to restart the REPL to clear the state otherwise you'll get conflicts....
include("../src/Framework.jl")

module Testing

using ..Framework

############################
# Struct TestInner 
############################
struct TestInner
    items::Vector{Int64}
end
const TESTINNER_SLOT_COUNT = 1
const TESTINNER_SLOT_MEM_SIZE = TESTINNER_SLOT_COUNT * sizeof(Int)
items(x::TestInner)::Vector{Int64} = x.items
function write_custom!(x::BufferDirect.Instance, pos::Int, value::TestInner, ::Type{TestInner})::Int
    start = pos
    slots = Vector{Int}(undef, TESTINNER_SLOT_COUNT)
    pos += sizeof(slots)
    pos = BufferDirect.write!(x, pos, value.items, Vector{Int64})

    if (length(slots) > 0)
        bytes = pos - start
        slots[1] = bytes
        BufferDirect.write_fixed_size!(x, start, slots, Vector{Int})
    end
    return pos
end
function read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestInner})::TestInner
    pos[] = pos[] + sizeof(TESTINNER_SLOT_MEM_SIZE)
    items = BufferDirect.read(x, pos, Vector{Int64})
    TestInner(items)
end

############################
# Struct TestInner_Buffer
############################
struct TestInner_Buffer
    _buffer::BufferDirect.Instance
    _start_pos::Int  # TODO:  Not sure I need this as I could just splice the buffer.
    TestInner_Buffer(buffer::BufferDirect.Instance, start_pos::Int) = new(buffer, start_pos)
end
function get_elem_size(x::BufferDirect.Instance, elem::TestInner_Buffer)::Int
    slots = BufferDirect.read_fixed_size(elem._buffer, Ref(elem._start_pos), TESTINNER_SLOT_COUNT, Vector{Int64})
    size = slots[1]
    size
end
function read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestInner_Buffer})::TestInner_Buffer
    # For the read we simply return the Buffer for the starting pos...
    TestInner_Buffer(x, pos[])
end
function items(x::TestInner_Buffer)::BufferedArray.Instance{Int64}
    start_pos = Ref(x._start_pos + TESTINNER_SLOT_MEM_SIZE)
    BufferDirect.read(x._buffer, start_pos, BufferedArray.Instance{Int64})
end
function to_obj(x::TestInner_Buffer)::TestInner
    items_raw = items(x)
    vec = BufferedIter.to_vector(BufferedIter.Instance{Int64}(items_raw))
    TestInner(vec)
end

############################
# Struct TestInner_Buffer
############################

struct TestOuter
    items::Vector{TestInner}
end

const TESTOUTER_SLOT_COUNT = 1
const TESTOUTER_SLOT_MEM_SIZE = TESTINNER_SLOT_COUNT * sizeof(Int)

items(x::TestOuter)::Vector{TestInner} = x.items
function write_custom!(x::BufferDirect.Instance, pos::Int, value::TestOuter, ::Type{TestOuter})::Int
    start = pos
    slots = Vector{Int}(undef, TESTINNER_SLOT_COUNT)

    pos += sizeof(slots)
    pos = BufferDirect.write!(x, pos, value.items, Vector{TestInner})

    if (length(slots) > 0)
        bytes = pos - start
        slots[1] = bytes
        BufferDirect.write_fixed_size!(x, start, slots, Vector{Int})
    end
    return pos
end
function read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestOuter})::TestOuter
    pos[] = pos[] + TESTOUTER_SLOT_MEM_SIZE
    vec = BufferDirect.read(x, pos, Vector{TestInner})
    TestOuter(vec)
end

struct TestOuter_Buffer
    _buffer::BufferDirect.Instance
    _start_pos::Int  # TODO:  Not sure I need this as I could just splice the buffer.
    TestOuter_Buffer(buffer::BufferDirect.Instance, start_pos::Int) = new(buffer, start_pos)
end
function read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestOuter_Buffer})::TestOuter_Buffer
    # For the read we simply return the Buffer for the starting pos...
    TestOuter_Buffer(x, pos[])
end
function get_elem_size(_::BufferedIter.Instance{T}, elem::TestOuter_Buffer)::Int where {T<:TestInner_Buffer}
    size = BufferDirect.read(elem._buffer, elem._start_pos, Int64)
    size
end
function items(x::TestOuter_Buffer)::BufferedArray.Instance{TestInner_Buffer}
    start_pos = Ref(x._start_pos + TESTOUTER_SLOT_MEM_SIZE)
    # BufferedArray.Instance{TestInner_Buffer}(x._buffer, start_pos[])
    BufferDirect.read(x._buffer, start_pos, BufferedArray.Instance{TestInner_Buffer})
end
function to_obj(x::TestOuter_Buffer)::TestOuter
    index = 1
    items_raw = BufferedIter.as_iter(items(x))
    inner_vec = Vector{TestInner}(undef, items_raw._count)
    while BufferedIter.has_next(items_raw)
        inner = BufferedIter.next(items_raw)
        inner_vec[index] = to_obj(inner)
        index += 1
    end

    TestOuter(inner_vec)
end

# Declare functions as part of the ExampleSerialize module.
# TODO: Not sure how to do with with my generated code yet.

Main.Framework.BufferDirect.get_elem_size(x::BufferDirect.Instance, elem::TestInner_Buffer)::Int = get_elem_size(x, elem)
Main.Framework.BufferDirect.get_elem_size(x::BufferDirect.Instance, elem::TestOuter_Buffer)::Int = get_elem_size(x, elem)

Main.Framework.BufferDirect.write_custom!(x::BufferDirect.Instance, pos::Int, value::TestInner, ::Type{TestInner})::Int = write_custom!(x, pos, value, TestInner)
Main.Framework.BufferDirect.write_custom!(x::BufferDirect.Instance, pos::Int, value::TestOuter, ::Type{TestOuter})::Int = write_custom!(x, pos, value, TestOuter)

Main.Framework.BufferDirect.read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestInner})::TestInner = read_custom(x, pos, TestInner)
Main.Framework.BufferDirect.read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestInner_Buffer})::TestInner_Buffer = read_custom(x, pos, TestInner_Buffer)
Main.Framework.BufferDirect.read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestOuter})::TestOuter = read_custom(x, pos, TestOuter)
Main.Framework.BufferDirect.read_custom(x::BufferDirect.Instance, pos::Ref{Int}, ::Type{TestOuter_Buffer})::TestOuter_Buffer = read_custom(x, pos, TestOuter_Buffer)

end # End Module Testing

using ..Framework
using ..Testing

buf = BufferDirect.Instance(512)
ti = Testing.TestInner([1, 2, 3, 4, 5])
BufferDirect.write!(buf, 1, ti, Testing.TestInner)

read_pos = Ref(1)
read_ti = BufferDirect.read(buf, read_pos, Testing.TestInner)
@assert Testing.items(ti) == Testing.items(read_ti)

tbi = Testing.TestInner_Buffer(buf, 1)
tbi_obj = Testing.to_obj(tbi)

@assert BufferDirect.read(buf, Ref(1), Int64) == 56 "Pos 1 should be 56 which is the total size of the Serialized TestInner struct"
@assert BufferDirect.read(buf, Ref(9), Int64) == 5 "Pos 9 should be 5 which is the number of items in the TestInner.Items Vector"
@assert Testing.items(ti) == Testing.items(tbi_obj)

to = Testing.TestOuter(
    [Testing.TestInner([1, 2, 3, 4, 5]),
    Testing.TestInner([6, 7, 8, 9, 10])]
)

BufferDirect.write!(buf, 1, to, Testing.TestOuter)
read_to = BufferDirect.read(buf, Ref(1), Testing.TestOuter)

@assert BufferDirect.read(buf, Ref(1), Int64) == 128 "Pos 9 should be 152 which is the total size of TestOuter"

tob = Testing.TestOuter_Buffer(buf, 1)
tob_obj = Testing.to_obj(tob)
@assert string(to) == string(tob_obj)


to2 = Testing.TestOuter(
    [Testing.TestInner([1, 2, 3]),      # 6
    Testing.TestInner([4, 5, 6, 7, 8]), # 30
    Testing.TestInner([9, 10, 11, 12])] # 42
)

BufferDirect.write!(buf, 1, to2, Testing.TestOuter)
tob2 = Testing.TestOuter_Buffer(buf, 1)
tob_obj2 = Testing.to_obj(tob2)
println(tob_obj2)


# Timed: (value = 161, time = 1.91e-5, bytes = 1296, gctime = 0.0, gcstats = Base.GC_Diff(1296, 0, 0, 19, 0, 0, 0, 0, 0))
@timed BufferDirect.write!(buf, 1, to2, Testing.TestOuter)

function sum_inners(buffer::BufferDirect.Instance)
    sum = 0

    tob = Testing.TestOuter_Buffer(buffer, 1)
    tob_iter = BufferedIter.as_iter(Testing.items(tob))
    while BufferedIter.has_next(tob_iter)
        inner = BufferedIter.next(tob_iter)
        inner_iter = BufferedIter.as_iter(Testing.items(inner))
        while BufferedIter.has_next(inner_iter)
            v = BufferedIter.next(inner_iter)
            sum += v
        end
    end

    sum
end

# Timed: (value = 78, time = 5.1e-6, bytes = 1712, gctime = 0.0, gcstats = Base.GC_Diff(1712, 0, 0, 41, 0, 0, 0, 0, 0))
@timed sum_inners(buf)


# Timed: time = 7.2e-6, bytes = 896, gctime = 0.0, gcstats = Base.GC_Diff(896, 0, 0, 12, 0, 0, 0, 0, 0)
function get(buffer::BufferDirect.Instance)
    BufferDirect.read(buffer, Ref(1), Testing.TestOuter)
end
@timed _ = get(buf)

# Timed: time = 1.25e-5, bytes = 2400, gctime = 0.0, gcstats = Base.GC_Diff(2400, 0, 0, 51, 0, 0, 0, 0, 0)
@timed Testing.to_obj(Testing.TestOuter_Buffer(buf, 1))


using Serialization

io = IOBuffer()

#Timed: (value = nothing, time = 3.81e-5, bytes = 2096, gctime = 0.0, gcstats = Base.GC_Diff(2096, 0, 0, 18, 0, 0, 0, 0, 0))
@timed serialize(io, to2)

seekstart(io)
-
#Timed: time = 4.22e-5, bytes = 1888, gctime = 0.0, gcstats = Base.GC_Diff(1888, 0, 0, 29, 0, 0, 0, 0, 0)
@timed deserialize(io)
