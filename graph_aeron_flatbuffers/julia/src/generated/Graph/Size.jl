# automatically generated by the FlatBuffers compiler, do not modify

Graph.eval(quote


FlatBuffers.@STRUCT struct Size
    value::Float64
end
FlatBuffers.@ALIGN(Size, 8)

Size(buf::AbstractVector{UInt8}) = FlatBuffers.read(Size, buf)
Size(io::IO) = FlatBuffers.deserialize(io, Size)

end)

