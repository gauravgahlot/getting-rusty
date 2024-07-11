pub fn parse(input: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn test_parse() {
        let res = parser::parse(String::from("{}"));
        assert_eq!(true, res);
    }
}

