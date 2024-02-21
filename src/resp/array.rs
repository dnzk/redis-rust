pub struct Array {
    pub elements: Vec<String>,
}

impl<'a> Array {
    pub fn from(source: &'a str) -> Self {
        let mut current = 0;
        let mut elements: Vec<String> = vec![];
        while let Some(c) = source.chars().nth(current) {
            match c {
                '\r' | '\n' | '$' => current += 1,
                n if n.is_ascii_digit() => current += 1,
                _ => {
                    let start = current;
                    while let Some(peek) = source.chars().nth(current + 1) {
                        if peek == '\r' {
                            current += 1;
                            break;
                        }
                        current += 1;
                    }
                    elements.push(String::from(&source[start..current]));
                }
            }
        }
        Array { elements }
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn creates_array_from_source() {
        let source = "2\r\n$4\r\necho\r\n$3\r\nhey\r\n";
        let array = Array::from(source);
        assert_eq!(array.elements.len(), 2);
        assert_eq!(array.elements[0], "echo");
        assert_eq!(array.elements[1], "hey");
    }
}
