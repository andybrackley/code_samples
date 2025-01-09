# types_gen

## Contents

- [types\_gen](#types_gen)
  - [Contents](#contents)
  - [Description](#description)
  - [Dependencies](#dependencies)
    - [Parsing](#parsing)
    - [Generation](#generation)
  - [TODO List:](#todo-list)
    - [Julia](#julia)

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

It relies on some framework code that is currently stored under the generated/{language}/framework directory.  This is currently hand written and copied into that directory manually

To generate the code run the tests under the tests sub-folder

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

To populate these macros in code see the .\src\askama\{language} directory.  This will contain a struct with a reference to the template file i.e.

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


## TODO List:

- [ ] - Offset calculations
- [ ] - Handling of variable length types

### Julia
- [ ] - Output enum
- [ ] - Output alias
- [ ] - Output const expressions
- [ ] - Equality / HashCode....
- [ ] - Dictionary ???
- [ ] - Other