pub struct StringConcatLint;

impl StringConcatLint {
    pub fn check_loop_body(&self, body: &str) -> bool {
        body.contains("String::") || body.contains(".push_str(") || body.contains("+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_concat_lint() {
        let lint = StringConcatLint;
        assert!(lint.check_loop_body("s.push_str(\"foo\")"));
        assert!(!lint.check_loop_body("do_something()"));
    }
}
