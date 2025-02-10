
struct Option{T}
    v::Union{T,Nothing}
    Option{T}(v::Union{T,Nothing}) where {T} = new{T}(v)
end
Some{T}(v::T) where {T} = Option{T}(v)
None(::Type{T}) where {T} = Option{T}(nothing)
is_none(x::Option{T}) where {T} = isnothing(x.v)
is_some(x::Option{T}) where {T} = !is_none(x)

function match(x::Option{T}, some::Function, none::Function) where {T}
    if is_some(x)
        some(x)
    else
        none(x)
    end
end
get(x::Option{T}) where {T} = match(x, x -> x.v, x -> error("Optional value is None"))




xo = Some(10)
xn = None(Int64)
xa = None(Any)
xb = None(Int32)

get(xo)
get(xn)

function test(x::Option{T}) where {T}

end

function test2(x::Option{Int64})

end


test2(xo)
test2(xn)
test2(xa)
test2(xb)