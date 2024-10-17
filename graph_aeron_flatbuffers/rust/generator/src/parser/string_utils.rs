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
                if(!in_braces) {
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