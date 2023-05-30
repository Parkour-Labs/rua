//! Utilities for rua_gen

/// Convert string to snake case, camel case, pascal case
pub trait RuaCaseConverter {
    /// Convert string to snake case
    fn to_snake_case(&self) -> String;
    /// Convert string to camel case
    fn to_camel_case(&self) -> String;
    /// Convert string to pascal case
    fn to_pascal_case(&self) -> String;
}

impl<T: AsRef<str>> RuaCaseConverter for T {
    fn to_snake_case(&self) -> String {
        let chars = self.as_ref().chars();
        let first = match chars.clone().next() {
            None => String::new(),
            Some(c) => c.to_ascii_lowercase().to_string(),
        };
        let rest = chars
            .map(|c| {
                if c.is_ascii_uppercase() {
                    "_".to_string() + &c.to_ascii_lowercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");
        first + rest.as_str()
    }

    fn to_camel_case(&self) -> String {
        // uncapitalize first letter and then capitalize the letters after
        // underscore
        let res = self
            .as_ref()
            .split("_")
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => {
                        c.to_ascii_uppercase().to_string() + chars.as_str()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("");
        let mut chars = res.chars();
        let first = match chars.next() {
            None => String::new(),
            Some(c) => c.to_ascii_lowercase().to_string(),
        };
        first + chars.as_str()
    }

    fn to_pascal_case(&self) -> String {
        let tmp = self.to_camel_case();
        let mut chars = tmp.chars();
        let first = match chars.next() {
            None => String::new(),
            Some(c) => c.to_ascii_uppercase().to_string(),
        };
        first + chars.as_str()
    }
}
