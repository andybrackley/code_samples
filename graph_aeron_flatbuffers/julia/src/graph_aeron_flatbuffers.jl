# https://docs.julialang.org/en/v1/




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

io = IOBuffer()
FlatBuffers.serialize(io, bookUpdate)

bytes = take!(io)
inflated = BookUpdate(bytes)


instId = inflated.id

println("Inflated: $instId")

println("Completed")

# end
