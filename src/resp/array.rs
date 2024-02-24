pub struct Array {
    pub elements: Vec<String>,
}

impl<'a> Array {
    pub fn from(source: &'a str) -> Self {
        let mut current = 0;
        let mut elements: Vec<String> = vec![];
        while let Some(c) = source.chars().nth(current) {
            match c {
                '\r' | '\n' => current += 1,
                '$' => {
                    current = skip(current, &source, '\r');
                }
                _ => {
                    let start = current;
                    current = skip(current, &source, '\r');
                    elements.push(String::from(&source[start..current]));
                }
            }
        }
        if let Ok(_) = elements[0].parse::<usize>() {
            elements = elements[1..].to_vec();
        }
        Array { elements }
    }
}

fn skip(mut current: usize, source: &str, on: char) -> usize {
    while let Some(peek) = source.chars().nth(current + 1) {
        if peek == on {
            current += 1;
            break;
        }
        current += 1;
    }
    current
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

    #[test]
    fn parses_multiple_words_as_value() {
        let source = "3\r\n$3\r\nset\r\n$3\r\nhey\r\n$10\r\nmy name is\r\n";
        let array = Array::from(source);
        assert_eq!(array.elements.len(), 3);
        assert_eq!(array.elements[0], "set");
        assert_eq!(array.elements[1], "hey");
        assert_eq!(array.elements[2], "my name is");
    }
}
