include("messages/book_update.jl")
include("../serialize_raw/src/generated/book_update.jl")

function test(items:: Vector{T}) where {T}
    typename = string(T)
    scalarStream = IOBuffer()
 
    for element in items
        serialize(scalarStream, element)
    end

    filename = "..\\serialized\\adhoc\\julia.scalars.$typename.bin"
    open(filename, "w") do file 
        write(file, take!(scalarStream))
    end
            
    readBuffer = IOBuffer()
    open(filename, "r") do file
        write(readBuffer, read(file))
    end
            
    bytes = take!(readBuffer)
    off::Int32 = 0

    new_vec = Vector{T}()
    for i in 1:length(items)
        item, off = deserialize(bytes, off, T)
        push!(new_vec, item)
    end

    isSame = items == new_vec
    @assert isSame
    println("Read Vec{$typename}::$new_vec")
end

test(['a','b','c','d'] )
test([ Int8(10), Int8(100), Int8(120), Int8(127),])
test([ Int32(10), Int32(100), Int32(1000), Int32(999999),])
test([ Int64(10), Int64(100), Int64(1000), Int64(999999),])
test(["Test1","Test2","Test3","Test4","Test5"])

optVec::Vector{Optional{Int64}} = [
    100,
    nothing, 
    200,
    nothing,
    999999,
    nothing,
    12345567890
]
test(optVec) 
