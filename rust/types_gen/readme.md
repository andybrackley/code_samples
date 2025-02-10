# types_gen

## Contents

- [types\_gen](#types_gen)
  - [Contents](#contents)
  - [Description](#description)
    - [Field Offsets](#field-offsets)
      - [Compile Time Offsets](#compile-time-offsets)
      - [Dynamic Offsets](#dynamic-offsets)
  - [Dependencies](#dependencies)
    - [Parsing](#parsing)
    - [Generation](#generation)
  - [Serialization Structure](#serialization-structure)
    - [Vector / String](#vector--string)
    - [Union { T, U }](#union--t-u-)
  - [TODO List](#todo-list)
    - [Julia](#julia)
  - [Similar Frameworks](#similar-frameworks)
    - [Google Protobuf](#google-protobuf)
    - [FlatBuffers](#flatbuffers)
    - [Cap'n'Proto](#capnproto)
    - [SBE](#sbe)
    - [Fast Binary Encoding (FBE)](#fast-binary-encoding-fbe)
  - [Additional Resources](#additional-resources)
    - [Comparisons](#comparisons)

## Description

The types_gen project converts julia types and generates Julia, Rust and C++ types and serialization code.

Currently it supports the following:

- structs
- enums
- alias
- const expressions
- abstract types

Given the following idl:

    mutable struct GraphMessageId
        id::UInt64
    end

It will generate a struct definition with getter functions.
If the struct is mutable it will also generate setter functions as well.  

You'll also get the serialize and deserialize functions (Format subject to change):

    mutable struct GraphMessageId
       _id::UInt64

    GraphMessageId(
        id::UInt64,
    ) = new(
            id,
        )
    end
    function serialize(self::GraphMessageId, buf::Bytes, start_pos::Int64) 
        offsets = []
        pos = start_pos + sizeof(offsets)
        pos = serialize(buf, pos, self._id)
        serialize(buffer, start_pos, offsets)
        return pos
    end

    function deserialize(buf::Bytes, pos::Ref{Int}, ::Type{T}) where { T<:GraphMessageId }
        offsets = []
        pos[] += sizeof(offsets)
        GraphMessageId(
            deserialize(buf, pos, UInt64), # Deserialize id
        )
    end
    id(self::GraphMessageId) = self._id 
    id!(self::GraphMessageId, value::UInt64) = self._id = value

Buffered Version

    struct GraphMessageId_Buffer
        _buffer::Vector{UInt8}
        _start_pos::UInt64
        new(buffer, start_pos)
    end

    # Offset Calculations
    const GraphMessageId_OFFSET_COUNT = 0
    const GraphMessageId_START_OFFSET = sizeof(Int) * GraphMessageId_OFFSET_COUNT
    const GraphMessageId_ID_OFFSET = GraphMessageId_START_OFFSET

    function get_id(self::GraphMessageId_Buffer)
        pos = self._start_pos + GraphMessageId_ID_OFFSET
        return deserialize(self._buffer, pos, UInt64)
    end

It relies on some framework code that is currently stored under the generated/{language}/framework directory.  This is currently hand written and copied into that directory manually

To generate the code run the tests under the tests sub-folder

### Field Offsets

The fields contained in the generated structs rely on positions within the buffer, this allows direct access into memory.

There are two types of offset

- Compile Time Offsets
- Dynamic Offsets

#### Compile Time Offsets

These are known at compile time and can be calculated by using the previous field offset + sizeof(previous field type)

#### Dynamic Offsets

These are calculated at runtime when the structure is serialized to the buffer,
they are not known at compile time because they will follow data types such as Vector or String which are variable length

The code generation will serialize these AFTER all the fixed sized fields to reduce the size of serialized data and for lookup performance.

## Dependencies

### Parsing

For parsing of the julia idl definitions I'm using the nom parser
[Docs](https://docs.rs/nom/latest/nom/)
[Readme](https://github.com/rust-bakery/nom)
[Crate](https://crates.io/crates/nom)

### Generation

For code generation I'm using the Askama package
[Readme](https://github.com/rinja-rs/askama/blob/main/README.md)

It has good support for templating and provides compile time checking for the template and the code
See here for the Template Syntax:
[TemplateSyntax](https://rinja-rs.github.io/askama/template_syntax.html)

The code generations templates are stored in the ./templates directory and are .txt files.  The files contain macros `see the Askama docs`.  

To populate these macros in code see the `.\src\askama\{language}` directory.  This will contain a struct with a reference to the template file i.e.

    #[derive(Template)]
    #[template(path = "julia_template_struct.txt", block = "struct_def")]
    struct StructJuliaDefTemplate<'a> {
        pub struct_def: &'a StructDefDetails,
    }

NOTE:

I'm currently using a local version that was built from their main branch
   The current release version doesn't have support for template blocks i.e.

      {%- block struct_def -%}
      {%- endblock -%}

You can work around this issue by either defining each block in a separate file or by not using blocks at all.

You can easily set this up locally by cloning the Askama repo into
   D:\third_party_repos\rust directory
If you don't currently have a D: you can into any directory and then adjust the Cargo.toml to point at this directory structure.

## Serialization Structure

[ Size of Object ]
[ Array Dynamic Offset Positions ]
[ Fixed Size Field Data ]
[ Var Size Field Data ]

### Vector / String

[ Vec Length ] [ Padding for Alignment boundaries ] [ Vec Items ]

### Union { T, U }

[ Selected Option Index ] [ Union Data ] [ Padding to largest size ]

## TODO List

- [ ] - Offset calculations
- [ ] - Handling of variable length types

### Julia

- [ x ] - Output enum
- [ ] - Output alias
- [ ] - Output const expressions
- [ ] - Equality / HashCode....
- [ ] - Dictionary ???
- [ ] - Other

## Similar Frameworks

### Google Protobuf

- [Docs](https://protobuf.dev/)
- [GitHub](https://github.com/protocolbuffers/protocolbuffers.github.io/blob/main/content/_index.md)

### FlatBuffers

- [Docs](https://flatbuffers.dev/)
- [GitHub](https://github.com/google/flatbuffers)
- [StackOverflow](https://stackoverflow.com/search?q=flatbuffers)
- [Rust](https://flatbuffers.dev/flatbuffers_guide_use_rust.html)
- [Rust-Example](https://github.com/google/flatbuffers/blob/master/tests/rust_usage_test/bin/monster_example.rs)
- [Julia](https://flatbuffers.juliadata.org/stable/)

### Cap'n'Proto

- [Docs](https://capnproto.org/)
- [GitHub](https://github.com/capnproto/capnproto)

### SBE

- [GitHub](https://github.com/real-logic/simple-binary-encoding)
- [CSharp User Guide](https://github.com/real-logic/simple-binary-encoding/wiki/Csharp-User-Guide)
- [Java User Guide](https://www.baeldung.com/java-sbe)
- [Julia-SBE](https://github.com/New-Earth-Lab/SimpleBinaryEncoding.jl)

### Fast Binary Encoding (FBE)

- [GitHub](https://github.com/chronoxor/FastBinaryEncoding/blob/master/proto/proto.fbe)

## Additional Resources


### Comparisons

- [chronoxor_github](https://github.com/chronoxor/CppSerialization)
