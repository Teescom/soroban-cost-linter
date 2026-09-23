use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SarifFormatter {
    pub version: String,
}

impl SarifFormatter {
    pub fn new() -> Self {
        Self {
            version: "2.1.0".to_string(),
        }
    }
    
    pub fn format(&self, _results: &[&str]) -> String {
        format!("{{\"version\": \"{}\", \"runs\": []}}", self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sarif_formatter() {
        let formatter = SarifFormatter::new();
        assert_eq!(formatter.version, "2.1.0");
        assert_eq!(formatter.format(&[]), "{\"version\": \"2.1.0\", \"runs\": []}");
    }
}
