include("common.jl")

module SerializeRaw

const Scalar = Main.Scalar
const IdTypes = Main.IdTypes
const Bytes = Main.Bytes
const Optional = Main.Optional

function serialize(stream::IO, i:: T) where {T <: Scalar } 
    write(stream, i) 
 end

 function serialize(stream::IO, i:: T) where {T <: Enum}
    write(stream, i) 
 end
 
 function serialize(stream::IO, i:: T) where { T <: AbstractString }
    len::Int32 = length(i)
    write(stream, len)
    write(stream, i) 
 end
 
function serialize(stream::IO, vec:: Vector{T}) where {T} 
    sizeOfVec = length(vec)

    write(stream, Int32(sizeOfVec))
    for (index, l) in enumerate(vec) 
        serialize(stream, l)
    end
end

function serializeAsOption(stream::IO, optional::Optional{T}) where {T}
    if isnothing(optional)
        serialize(stream, Char(0)) # Write Null to stream so we can identify this when we read it back
    else
        # If the value isn't nothing we'll just write the value to the stream
        # As long as the value isn't a Null this will work.
        # Unfortunately I'm having some issues in Julia where Union types such as Optional
        # get erased and you're left with just the underlying type.
        # I think this will come back as an issue when I look at implementing full Unions 

        serialize(stream, Char(1)) # Write a value to stream to align with the Null.  TODO: We can probably figure out in the deserialize we have a value based on it not being null
        serialize(stream, optional)
    end

    # streamPos = position(stream)
end

function serialize(stream::IO, optional::Optional{T}) where {T} 
    return serializeAsOption(stream, optional)
end

end # End Module SerializeRaw

function serialize(stream::IO, element::T) where {T}
    SerializeRaw.serialize(stream, element)
end

function selectFunction(::Type{T}) where {T} 
    return SerializeRaw.serialize::Function
end

function selectFunction(::Type{Optional{T}}) where {T} 
    return SerializeRaw.serializeAsOption::Function
end

function serialize(stream::IO, elements::Vector{T}) where {T}
    # isUnion = isunionwithnothing(T)
    serializer = selectFunction(T)

    vector_len = length(elements)
    serialize(stream, vector_len)

    for element in elements
        serializer(stream, element)
    end
end