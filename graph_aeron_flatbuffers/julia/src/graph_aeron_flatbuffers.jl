# https://docs.julialang.org/en/v1/



# Override our path to FlatBuffers as it currently has a bug
# when trying to deserialize the BookUpdate struct
flatBufferPath = pwd() * "\\FlatBuffers"
if !(flatBufferPath in LOAD_PATH)
   pushfirst!(LOAD_PATH, flatBufferPath)
end


# module graph_aeron 

import FlatBuffers;
include("generated/book_generated.jl");

using .Graph: Exchange, ExchangeDeribit, Timestamp, InstrumentId, Level, BookUpdate, BookUpdateType, BookUpdateTypeUpdate, BookUpdateTypeSnapshot

bookUpdate::BookUpdate = BookUpdate(
    Timestamp(10),
    Timestamp(10),
    InstrumentId(ExchangeDeribit, "12345"),
    Vector{Level}(), 
    Vector{Level}(),
    BookUpdateTypeUpdate
)

# See: https://github.com/rjkat/flatbuffers-julia/blob/master/samples/sample_binary.jl

for i in 1:5
    io = IOBuffer()
    FlatBuffers.serialize(io, bookUpdate)

    bytes = take!(io)
    BookUpdate(bytes)
end

io = IOBuffer()
timeSerialize = @timed FlatBuffers.serialize(io, bookUpdate)

println("FlatBuffers::SerializeTime::$timeSerialize")

bytes = take!(io)
timeDeserialize = @timed BookUpdate(bytes)


println("FlatBuffers::DeserializeTime::$timeDeserialize")


instId = timeDeserialize.value.id

println("Inflated: $instId")

println("Completed")

# end
