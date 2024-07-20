pub mod string_utils {
    pub fn camel_to_snake(input: &str) -> String {
        let mut snake_case = String::with_capacity(input.len());
        for (i, c) in input.chars().enumerate() {
            if c.is_uppercase() {
                if i != 0 {
                    snake_case.push('_');
                }
                snake_case.extend(c.to_lowercase());
            } else {
                snake_case.push(c);
            }
        }
        snake_case
    }
}

#[cfg(test)]
mod tests {
    use super::string_utils::camel_to_snake;

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("CamelCase"), "camel_case");
        assert_eq!(camel_to_snake("camelCase"), "camel_case");
        assert_eq!(camel_to_snake("CamelCaseString"), "camel_case_string");
        assert_eq!(camel_to_snake("camelCaseString"), "camel_case_string");
        assert_eq!(camel_to_snake("Camel"), "camel");
        assert_eq!(camel_to_snake("camel"), "camel");
        assert_eq!(camel_to_snake("HTTPRequest"), "http_request");
    }
}
