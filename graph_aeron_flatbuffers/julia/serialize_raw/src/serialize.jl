include("common.jl")

# const Scalar = Main.Scalar
# const IdTypes = Main.IdTypes
# const Bytes = Main.Bytes

function serialize(stream::IO, i:: T) where {T <: Scalar } 
    type = string(T)
    size = sizeof(T)

    println("Type: $type, size: $size, value: $i")
    write(stream, i) 
 end

 function serialize(stream::IO, i:: T) where {T <: Enum}
    type = string(T)
    size = sizeof(T)

    println("Enum::Type: $type, size: $size, value: $i")
    write(stream, i) 
 end
 
 function serialize(stream::IO, i:: T) where { T <: AbstractString }
    type = string(T)
    len = length(i)
    # size = sizeof(T)

    println("String::Type: $type, length: $len, value: $i")
    
    write(stream, len)
    write(stream, i) 
 end
 
function serialize(stream::IO, vec:: Vector{T}) where {T} 
    typename = string(T)
    sizeOfVec = length(vec)

    println("Size: $sizeOfVec, Type: $typename")
    write(stream, Int32(sizeOfVec))

    for (index, l) in enumerate(vec) 
        serialize(stream, l)
    end
end

