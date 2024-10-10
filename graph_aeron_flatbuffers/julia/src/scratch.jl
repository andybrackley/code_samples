include("messages/book_update.jl")
include("../serialize_raw/src/generated/book_update.jl")

function run_serialize_test(update::BookUpdate, stream::IOBuffer) 
    write(stream, Int32(0))
    size = serializeBookUpdate(stream, t)
    seekstart(stream)
    write(stream, Int32(size))
    return stream    
end

function run_deserialize_test(bytes::Bytes)
    deserializeBookUpdate(bytes)
end

bids:: Vector{Level} = [ Level(1), Level(2), Level(3) ]
asks:: Vector{Level} = [ Level(4), Level(3), Level(2) ]

t = BookUpdate(Timestamp(100), nothing, InstrumentId(ExchangeDeribit, "InstId::1234"), BookUpdateTypeSnapshot, bids, asks)

for i in 1:5 
    stream = IOBuffer()
    run_serialize_test(t, stream)
end

stream = IOBuffer()
stream_timed = @timed run_serialize_test(t, stream)
println("timed::run_serialize_test::$stream_timed")

filename = "..\\serialized\\adhoc\\julia.bookupdate.bin"

open(filename, "w") do file 
    write(file, take!(stream))
end

println("Written $size bytes to stream")


readBuffer = IOBuffer()
open(filename, "r") do file
    write(readBuffer, read(file))
end
    
bytes = take!(readBuffer)

timed_deserialize = @timed run_deserialize_test(bytes)
println("timed::run_deserialize_test::$stream_timed")
