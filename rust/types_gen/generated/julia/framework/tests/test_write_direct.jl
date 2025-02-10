# NOTE: If you re-run you'll need to restart the REPL to clear the state otherwise you'll get conflicts....
include("../src/Framework.jl")

module Testing

using ..Framework


const TestDirectWrite_Inner_OFFSET_COUNT = 0
const TestDirectWrite_Inner_SLOT_COUNT = 1 + TestDirectWrite_Inner_OFFSET_COUNT
const TestDirectWrite_Inner_START_OFFSET = sizeof(Int64) * TestDirectWrite_Inner_SLOT_COUNT

const TestDirectWrite_Inner_ID_OFFSET = TestDirectWrite_Inner_START_OFFSET

struct TestDirectWrite_Inner
    buffer::BufferDirect.Instance
    start_pos::Int
end
function id(x::TestDirectWrite_Inner)::Int
    BufferDirect.read(x.buffer, Ref(x.start_pos + TestDirectWrite_Inner_ID_OFFSET), Int)
end
function id(x::TestDirectWrite_Inner, val::Int)
    BufferDirect.write!(x.buffer, x.start_pos + TestDirectWrite_Inner_ID_OFFSET, val, Int)
end

const TestDirectWrite_OFFSET_COUNT = 2
const TestDirectWrite_SLOT_COUNT = 1 + TestDirectWrite_OFFSET_COUNT
const TestDirectWrite_START_OFFSET = sizeof(Int64) * TestDirectWrite_SLOT_COUNT

# const TestDirectWrite_DESC_OFFSET = TestDirectWrite_START_OFFSET + sizeof(Int)
const TestDirectWrite_INNERS_OFFSET = TestDirectWrite_START_OFFSET + sizeof(Int)


struct TestDirectWrite
    buffer::BufferDirect.Instance
    start_pos::Int

    slots::Vector{Int64}
    TestDirectWrite(buffer::BufferDirect.Instance, start_pos::Int) = new(buffer, start_pos, Vector{Int64}(undef, TestDirectWrite_SLOT_COUNT))
end
function write_size(x::TestDirectWrite)::Int
    size = 0
    BufferDirect.write!(x.buffer, x.start_pos, size, Int)
end
# function desc(x::TestDirectWrite)::String
#     BufferDirect.read(x.buffer, Ref(x.start_pos + TestDirectWrite_DESC_OFFSET), String)
# end
# function desc(x::TestDirectWrite, val::String)
#     pos = x.start_pos + TestDirectWrite_DESC_OFFSET
#     pos = BufferDirect.write!(x.buffer, pos, val, String)

#     # Update Desc2 offset
#     write_size(x)
# end
function inners(x::TestDirectWrite)::BufferedArray.Instance
    BufferedArray.Instance(x.buffer, x.start_pos + TestDirectWrite_INNERS_OFFSET)
end
function inners(x::TestDirectWrite, items::BufferedArray.Instance)::BufferedArray.Instance

end

# function desc2(x::TestDirectWrite)::String
#     BufferDirect.read(x.buffer, Ref(x.start_pos + TestDirectWrite_DESC_OFFSET), String)
# end
# function desc2(x::TestDirectWrite, val::String)
#     pos = x.start_pos + TestDirectWrite_DESC_OFFSET
#     pos = BufferDirect.write!(x.buffer, pos, val, String)
#     write_size(x)
# end
# function desc3(x::TestDirectWrite)::String
#     BufferDirect.read(x.buffer, Ref(x.start_pos + TestDirectWrite_DESC_OFFSET), String)
# end
# function desc3(x::TestDirectWrite, val::String)
#     pos = x.start_pos + TestDirectWrite_DESC_OFFSET
#     pos = BufferDirect.write!(x.buffer, pos, val, String)
#     write_size(x)
# end

end

using ..Framework
using .Testing

buf = BufferDirect.Instance(256)
tdw = Testing.TestDirectWrite_Inner(buf, 1)
Testing.id(tdw, 5)
Testing.id(tdw)

container = Testing.TestDirectWrite(buf, 1)


# Testing.desc(tdw, "Desc")
# Testing.desc2(tdw, "Desc2")
# Testing.desc3(tdw, "Desc3")

# println("tdw(id) = ", Testing.id(tdw), ", twd(desc) = ", Testing.desc(tdw))

