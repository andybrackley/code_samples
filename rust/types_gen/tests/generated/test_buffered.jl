include("../../generated/julia/framework/src/Framework.jl")
include("../../generated/julia/lib.jl")

buf = BufferDirect.Instance(2048)

##################
# Write Testing
##################

function populate_header(gmh_hdr::BufferedObj.Writer{GraphMessageHeader})
    # Write fixed-sized fields....
    id!(gmh_hdr, GraphMessageId(UInt64(99)))
    conflate_count!(gmh_hdr, UInt64(111))

    # Create a 2 element array for the Parent Ids.
    parent_id_arr = parent_ids_writer(gmh_hdr)
    id1 = BufferedArray.create_item(parent_id_arr)
    id!(id1, UInt64(111))
    id2 = BufferedArray.create_item(parent_id_arr)
    id!(id2, UInt64(222))

    # Write Optionals
    ts_enq_writer = Framework.writer_for(ts_enqueued_writer(gmh_hdr), BufferedObj.Writer{Timestamp})
    stamp!(ts_enq_writer, UInt64(123))

    ts_in_writerx = Framework.writer_for(ts_in_writer(gmh_hdr), BufferedObj.Writer{Timestamp})
    stamp!(ts_in_writerx, UInt64(456))

    msg_writer = Framework.writer_for(msg_type_writer(gmh_hdr), String)
    Framework.set(msg_writer, "Hello World")

    # msg_writer = Framework.writer_for(msg_type_writer(gmh_hdr), Nothing)
    finish(gmh_hdr)
end

function create_header(buf::BufferDirect.Instance)
    gmh_hdr = BufferedObj.Writer{GraphMessageHeader}(buf, Ref(1))
    populate_header(gmh_hdr)
    return gmh_hdr
end

# timed: (time = 4.45e-5, bytes = 3584, gctime = 0.0, gcstats = Base.GC_Diff(3584, 0, 0, 65, 0, 0, 0, 0, 0))
@timed created = create_header(buf)

# timed: (time = 1.7e-6, bytes = 48, gctime = 0.0, gcstats = Base.GC_Diff(48, 0, 0, 1, 0, 0, 0, 0, 0)))
@timed gmh_hdr_rdr = GraphMessageHeader_Reader(buf, 1)

# timed: ( time = 4.36e-5, bytes = 2208, gctime = 0.0, gcstats = Base.GC_Diff(2208, 0, 0, 49, 0, 0, 0, 0, 0)))
@timed v = buffer_to_value(gmh_hdr_rdr)

function create_msg_direct(buf::BufferDirect.Instance)
    msg = BufferedObj.Writer{GraphMessageString}(buf, Ref(1))
    gmh = header_writer(msg)
    populate_header(gmh)

    Framework.set(data_writer(msg), "Hello World")
    finish(msg)
    msg
end

# timed: ( time = 4.82e-5, bytes = 4240, gctime = 0.0, gcstats = Base.GC_Diff(4240, 0, 0, 76, 0, 0, 0, 0, 0)) )
@timed msg_w = create_msg_direct(buf)

# timed: ( time = 5.29e-5, bytes = 2704, gctime = 0.0, gcstats = Base.GC_Diff(2704, 0, 0, 59, 0, 0, 0, 0, 0)) )
@timed msg_v = buffer_to_value(msg_w)

using Profile
@profile create_header(buf)
Profile.print()

# Test the Union Helper functions
idx = Framework.type_to_index(Optional{Int64}, Nothing)
type = Framework.index_to_type(Optional{Int64}, idx)
@assert type == Nothing

idx2 = Framework.type_to_index(Optional{Int64}, Int64)
type2 = Framework.index_to_type(Optional{Int64}, idx2)
@assert type2 == Int64

##########
# Value type serialization testing and timings....
##########

function create_header_v()
    hdr = GraphMessageHeader(
        GraphMessageId(UInt64(1)),
        [GraphMessageId(UInt64(2)), GraphMessageId(UInt64(3))],
        Timestamp(UInt64(5)),
        Timestamp(UInt64(6)),
        UInt64(7),
        "msg_type",
    )

    return hdr
end

function serialize_hdr(buf::BufferDirect.Instance, hdr::GraphMessageHeader)
    BufferDirect.write!(buf, 1, hdr, GraphMessageHeader)
end

function create_and_serialize(buf::BufferDirect.Instance)
    hdr = create_header_v()
    serialize_hdr(buf, hdr)
end

# timed: (time = 3.0e-6, bytes = 112, gctime = 0.0, gcstats = Base.GC_Diff(112, 0, 0, 4, 0, 0, 0, 0, 0)))
@timed t_hdr = create_header_v()

# timed: (value = 132, time = 2.42e-5, bytes = 2640, gctime = 0.0, gcstats = Base.GC_Diff(2640, 0, 0, 41, 0, 0, 0, 0, 0))
@timed serialize_hdr(buf, t_hdr)

# timed: (value = 132, time = 2.46e-5, bytes = 2640, gctime = 0.0, gcstats = Base.GC_Diff(2640, 0, 0, 43, 0, 0, 0, 0, 0))
@timed create_and_serialize(buf)

###########
#
###########

struct Test

end

buf = BufferDirect.Instance(128)

function hdr(buf::BufferDirect.Instance, f::Function)
    s = f(BufferedObj.Writer{Test}(buf, Ref(1)))
    return s
end

@timed hdr(buf, wr -> BufferDirect.write!(wr._buffer, wr._write_pos[], 1, Int64))



pos = Ref(1)
@timed p1 = pos[]

# Dereference of the ref is quite a lot slower
function test1()
    for i in 1:10000
        BufferDirect.write!(buf, 1, pos[], Int64)
    end
end

function test2()
    for i in 1:10000
        BufferDirect.write!(buf, 1, 1, Int64)
    end
end


# time = 0.0013406, bytes = 640000, gctime = 0.0, gcstats = Base.GC_Diff(640000, 0, 0, 10000, 0, 0, 0, 0, 0))
@timed test1()


# time = 0.0004535, bytes = 640000, gctime = 0.0, gcstats = Base.GC_Diff(640000, 0, 0, 10000, 0, 0, 0, 0, 0))
@timed test2()

###########