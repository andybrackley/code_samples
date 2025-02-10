include("../../generated/julia/framework/src/Framework.jl")
include("../../generated/julia/lib.jl")

buf = BufferDirect.Instance(256)

msg_id = GraphMessageId(UInt64(99))
pos = BufferDirect.write!(buf, 1, msg_id, GraphMessageId)

msg_id_read = BufferDirect.read(buf, Ref(1), GraphMessageId)
@assert msg_id == msg_id_read

msg_id_buf = BufferDirect.read(buf, Ref(1), GraphMessageId_Buffer)

function assert_equal(lhs::GraphMessageIdT, rhs::GraphMessageIdT)
    @assert lhs == rhs
    @assert id(lhs) == id(rhs) "read id: $lhs does not match original id: $rhs"
    println("lhs.id(", id(lhs), ")==rhs.id(", id(rhs), ")")
end

assert_equal(msg_id_buf, msg_id)
assert_equal(msg_id_buf, msg_id_read)

msg_id_arr = [
    GraphMessageId(UInt64(99)),
    GraphMessageId(UInt64(88)),
    GraphMessageId(UInt64(77)),
    GraphMessageId(UInt64(66))
]

pos = BufferDirect.write!(buf, 1, msg_id_arr, Vector{GraphMessageId})
msg_id_arr_read = BufferDirect.read(buf, Ref(1), Vector{GraphMessageId})

@assert msg_id_arr == msg_id_arr_read

msg_id_arr_buf = BufferDirect.read(buf, Ref(1), BufferedArray.Instance{GraphMessageId_Buffer})
msg_id_arr_buf_vec = buffer_to_value(msg_id_arr_buf)

@assert msg_id_arr_buf_vec == msg_id_arr_read



# th = TestHeader(UInt64(1),
#     [GraphMessageId(UInt64(2)), GraphMessageId(UInt64(3))],
#     GraphMessageId(UInt64(25)),
#     [GraphMessageId(UInt64(4)), GraphMessageId(UInt64(5))])


# buf = BufferDirect.Instance(256)
# BufferDirect.write!(buf, 1, th, TestHeader)
# th_read = BufferDirect.read(buf, Ref(1), TestHeader)





# BufferDirect.write!(buf, 1, [GraphMessageId(UInt64(1)), GraphMessageId(UInt64(2))], Vector{Int64})
# vec = BufferDirect.read(buf, Ref(1), Vector{GraphMessageId})
