pub fn split_preserving_braces<'a>(line: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_braces = false;

    for (i, ch) in line.chars().enumerate() {
        match ch {
            '{' => {
                in_braces = true;
            },

            '}' => {
                in_braces = false;
            },
            ' ' => {
                if !in_braces {
                    result.push(&line[start..i]);
                    start = i + 1;
                }
            },
            _ => {}
        }
    }

    if start < line.len() {
        result.push(&line[start..]);
    }

    result
}


pub fn to_camel_case(s: &str) -> String {
    let mut camel_case = String::new();
    let mut upper_next = true;
    for c in s.chars() {
        if c == '_' {
            upper_next = true;
        } else if upper_next {
            camel_case.push(c.to_ascii_uppercase());
            upper_next = false;
        } else {
            camel_case.push(c);
        }
    }
    camel_case
}
