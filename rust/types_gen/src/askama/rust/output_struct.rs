use askama::Template;
// src\common\askama\rust\output_struct.rs

/*

#[derive(Template)]
#[template(source = "{{ crate::templates::utils::foo(123) }}", ext = "txt")]
struct MyTemplate;

*/

struct Field<'a> {
    field: &'a str,
    typ: &'a str,
}

struct StructDefDetails<'a> {
    struct_name: &'a str,
    generic_args: String,
    fields: Vec<Field<'a>>,
}
impl<'a> StructDefDetails<'a> {
    pub fn new() -> Self {
        let args: Vec<String> = Vec::new();

        Self {
            struct_name: "GraphMessageId",
            generic_args: vec!["T".to_string()].join(", "),
            // generic_args: args.join(", "),
            fields: vec![
                Field { field: "field1", typ: "i64" },
                Field { field: "field2", typ: "u64" }
            ],
        }
    }
}

#[derive(Template)]
#[template(path = "rs_template_struct.txt", block = "struct_def")]
struct StructRustDefTemplate<'a> {
    struct_def: &'a StructDefDetails<'a>,
}

#[derive(Template)]
#[template(path = "cpp_template_struct.txt", block = "struct_def")]
struct StructCppDefTemplate<'a> {
    struct_def: &'a StructDefDetails<'a>,
}

#[cfg(test)]
pub mod test_rust_outputs {
    use askama::Template;

    use crate::askama::rust::output_struct::{ StructDefDetails, StructRustDefTemplate };

    #[test]
    pub fn test_struct_definition() {
        let details = StructDefDetails::new();

        let output = (StructRustDefTemplate {
            struct_def: &details,
        })
            .render()
            .unwrap();
        println!("{}", output);
    }
}

#[cfg(test)]
pub mod test_cpp_outputs {
    use askama::Template;
    use crate::askama::rust::output_struct::{ StructDefDetails, StructCppDefTemplate };

    #[test]
    pub fn test_struct_definition() {
        let details = StructDefDetails::new();

        let output = (StructCppDefTemplate {
            struct_def: &details,
        })
            .render()
            .unwrap();
        println!("{}", output);
    }
}

// {{~ "" -}}
// {% if !generic_args.is_empty() {{ generic_args }} %}
