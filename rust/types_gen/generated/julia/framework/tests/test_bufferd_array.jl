include("../src/Framework.jl")
include("../../lib.jl")

using ..Framework

buf = BufferDirect.Instance(256)

msgid = GraphMessageId(UInt64(99))
BufferDirect.write!(buf, 1, msgid, GraphMessageId)

msgid_read = BufferDirect.read(buf, Ref(1), GraphMessageId)

msgid_read_val = id(msgid_read)

msgid_buf_read = BufferDirect.read(buf, Ref(1), GraphMessageIdBuffer)
msgid_buf_id = id(msgid_buf_read)

function testing(lhs::GraphMessageIdT, rhs::GraphMessageIdT)
    @assert lhs == rhs
    @assert id(lhs) == id(rhs)
end

testing(msgid, msgid_read)
testing(msgid_read, msgid_buf_read)

msg_id_vec = [GraphMessageId(UInt64(99)), GraphMessageId(UInt64(77)), GraphMessageId(UInt64(66)), GraphMessageId(UInt64(55))]
BufferDirect.write!(buf, 1, msg_id_vec, Vector{GraphMessageId})

msg_id_vec_buf = BufferDirect.read(buf, Ref(1), BufferedArray.Instance{GraphMessageIdBuffer})
msg_id_vec_buf_val = buffer_to_value(msg_id_vec_buf)


# msgid = GraphMessageId(UInt64(99))

# buf = BufferDirect.Instance(256)
# BufferDirect.write!(buf, 1, msgid, GraphMessageId)

# msg_id_buf = BufferDirect.read(buf, Ref(1), GraphMessageId_Buffer)
# msg_id_buf_val = BufferDirect.buffer_to_value(msg_id_buf, GraphMessageId)

# function test(lhs::GraphMessageIdT, rhs::GraphMessageIdT)
#     println("lhs::id(", id(lhs), ") == rhs::id(", id(rhs), ")")
#     @assert lhs == rhs
# end

# test(msgid, msg_id_buf)
# test(msgid, msg_id_buf_val)

# msg_id_vec = [GraphMessageId(UInt64(99)), GraphMessageId(UInt64(77)), GraphMessageId(UInt64(66)), GraphMessageId(UInt64(55))]
# BufferDirect.write!(buf, 1, msg_id_vec, Vector{GraphMessageId})

# msg_id_vec_buf = BufferDirect.read(buf, Ref(1), BufferedArray.Instance{GraphMessageId_Buffer})
# msg_id_vec_buf_val = BufferDirect.buffer_to_value(msg_id_vec_buf, Vector{GraphMessageId})
# @assert msg_id_vec == msg_id_vec_buf_val

# msg_hdr = GraphMessageHeader(
#     GraphMessageId(UInt64(99)),
#     [GraphMessageId(UInt64(77)), GraphMessageId(UInt64(66)), GraphMessageId(UInt64(55))],
#     nothing,
#     nothing,
#     UInt64(4),
#     "test_hdr")

# BufferDirect.write!(buf, 1, msg_hdr, GraphMessageHeader)

# msg_hdr_read = BufferDirect.read(buf, Ref(1), GraphMessageHeader)

# msg_hdr_buf = BufferDirect.read(buf, Ref(1), GraphMessageHeader_Buffer)



# msg_hdr_buf_val = BufferDirect.buffer_to_value(msg_hdr_buf, GraphMessageHeader)

