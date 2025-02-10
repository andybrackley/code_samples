include("../../generated/julia/framework/src/Framework.jl")
include("../../generated/julia/lib.jl")

buf = BufferDirect.Instance(2048)

gm_id = GraphMessageId(UInt64(1))
pos = BufferDirect.write!(buf, 1, gm_id, GraphMessageId)
gm_id_read = BufferDirect.read(buf, Ref(1), GraphMessageId)

gm_id_read_prop = id(gm_id_read)
@assert gm_id._id == gm_id_read._id "read id: $gm_id_read does not match original id: $gm_id"
@assert gm_id._id == gm_id_read_prop "read id: $gm_id_read_prop does not match original id: $gm_id"

gm_id_buf = BufferDirect.read(buf, Ref(1), GraphMessageId_Reader)
gm_id_buf_prop = id(gm_id_buf)
@assert gm_id._id == gm_id_buf_prop "read id: $gm_id_buf_prop does not match original id: $gm_id"


@assert gm_id == gm_id_read

buf2 = BufferDirect.Instance(2048)
pos = BufferDirect.write!(buf2, 5, gm_id, GraphMessageId)
gm_id2_buf = BufferDirect.read(buf2, Ref(5), GraphMessageId_Reader)

@assert gm_id_buf == gm_id2_buf
@assert gm_id == gm_id2_buf
id(gm_id_buf)
id(gm_id2_buf)

gm_id_buf
gm_id2_buf


function printid(x::GraphMessageIdT)
    println(id(x))
end

printid(gm_id)
printid(gm_id_buf)
serialized_size(GraphMessageId)

hdr = GraphMessageHeader(
    GraphMessageId(UInt64(1)),
    [GraphMessageId(UInt64(2)), GraphMessageId(UInt64(3))],
    Timestamp(UInt64(5)),
    Timestamp(UInt64(6)),
    UInt64(7),
    "msg_type",
)

# Timed: (value = 132, time = 1.74e-5, bytes = 1584, gctime = 0.0, gcstats = Base.GC_Diff(1584, 0, 0, 25, 0, 0, 0, 0, 0))
result = @timed BufferDirect.write!(buf, 1, hdr, GraphMessageHeader)
println("TypesGen: Timing serialization of Write::GraphMessageHeader[$result]")

# Timed: (time = 4.1e-5, bytes = 2208, gctime = 0.0, gcstats = Base.GC_Diff(2320, 0, 0, 52, 0, 0, 0, 0, 0)))
result = @timed hdr_read = BufferDirect.read(buf, Ref(1), GraphMessageHeader)
println("TypesGen: Timing serialization of Read::GraphMessageHeader[$result]")

# Timed: (time = 4.2e-6, bytes = 144, gctime = 0.0, gcstats = Base.GC_Diff(144, 0, 0, 4, 0, 0, 0, 0, 0)))
result = @timed hdr_buf_read = BufferDirect.read(buf, Ref(1), GraphMessageHeader_Reader)
println("TypesGen: Timing serialization of Read::GraphMessageHeader_Reader[$result]")

################

# Julia Serialization timings:

using Serialization

io = IOBuffer()

#Timed: time = 3.88e-5, bytes = 1328, gctime = 0.0, gcstats = Base.GC_Diff(1328, 0, 0, 22, 0, 0, 0, 0, 0))
result = @timed serialize(io, hdr)
println("Julia: timing serialization of Write::GraphMessageHeader[$result]")

seekstart(io)
-
#Timed: time = time = 5.1e-5, bytes = 1816, gctime = 0.0, gcstats = Base.GC_Diff(1816, 0, 0, 30, 0, 0, 0, 0, 0))
result = @timed deserialize(io)
println("Julia: timing serialization of Read::GraphMessageHeader[$result]")


###############

msg = GraphMessageString(hdr, "data")

#timed: (value = 172, time = 2.2e-5, bytes = 1744, gctime = 0.0, gcstats = Base.GC_Diff(1744, 0, 0, 27, 0, 0, 0, 0, 0))
@timed BufferDirect.write!(buf, 1, msg, GraphMessageString)

#timed: time = 3.82e-5, bytes = 2568, gctime = 0.0, gcstats = Base.GC_Diff(2568, 0, 0, 59, 0, 0, 0, 0, 0))
@timed BufferDirect.read(buf, Ref(1), GraphMessageString)

#timed: time = 3.7e-6, bytes = 96, gctime = 0.0, gcstats = Base.GC_Diff(96, 0, 0, 3, 0, 0, 0, 0, 0))
@timed BufferDirect.read(buf, Ref(1), GraphMessageString_Reader)


msg_read_buf = BufferDirect.read(buf, Ref(1), GraphMessageString_Reader)

msg_hdr = header(msg_read_buf)
msg_read_parent_ids = parent_ids(msg_hdr)
msg_read_buf_val = buffer_to_value(msg_read_buf)

msg_read_parent_ids = buffer_to_value(msg_read_parent_ids)
@assert msg_read_parent_ids == parent_ids(hdr)

### Profile....

using Profile

@profile BufferDirect.write!(buf, 1, msg, GraphMessageString)

Profile.print()

###############