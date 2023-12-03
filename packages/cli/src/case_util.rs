use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub enum CaseType {
    SnakeCase,  // snake_case
    KebabCase,  // kebab-case
    CamelCase,  // camelCase
    PascalCase, // PascalCase
    Unknown,    // none
}

impl CaseType {
    pub fn from_str(value: &str) -> CaseType {
        let snake_case = value.is_case(Case::Snake);
        let kebab_case = value.is_case(Case::Kebab);
        let camel_case = value.is_case(Case::Camel);
        let pascal_case = value.is_case(Case::Pascal);

        if snake_case {
            return CaseType::SnakeCase;
        }
        if kebab_case {
            return CaseType::KebabCase;
        }
        if camel_case {
            return CaseType::CamelCase;
        }
        if pascal_case {
            return CaseType::PascalCase;
        }

        CaseType::Unknown
    }

    pub fn from_str_type(&self, value: &str) -> String {
        match self {
            CaseType::SnakeCase => {
                return value.to_case(Case::Snake);
            }
            CaseType::KebabCase => {
                return value.to_case(Case::Kebab);
            }
            CaseType::CamelCase => {
                return value.to_case(Case::Camel);
            }
            CaseType::PascalCase => {
                return value.to_case(Case::Pascal);
            }
            CaseType::Unknown => {
                return value.to_string();
            }
        };
    }

    pub fn is_not_unknown(&self) -> bool {
        self != &CaseType::Unknown
    }

    pub fn to_str_name(&self) -> &str {
        match self {
            CaseType::SnakeCase => "snake_case",
            CaseType::KebabCase => "kebab-case",
            CaseType::CamelCase => "camelCase",
            CaseType::PascalCase => "PascalCase",
            CaseType::Unknown => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_test_case() {
        let result = CaseType::from_str("hello_world");
        assert_eq!(result, CaseType::SnakeCase);

        let result = CaseType::from_str("hello-world");
        println!("{:?}", result);
        assert_eq!(result, CaseType::KebabCase);

        let result = CaseType::from_str("helloWorld");
        assert_eq!(result, CaseType::CamelCase);

        let result = CaseType::from_str("HelloWorld");
        assert_eq!(result, CaseType::PascalCase);

        let result = CaseType::from_str("helloWorld-");
        assert_eq!(result, CaseType::Unknown);

        let result = CaseType::from_str("helloWorld_");
        assert_eq!(result, CaseType::Unknown);

        let result = CaseType::from_str("helloWorld_");
        assert_eq!(result, CaseType::Unknown);

        let result = CaseType::from_str("helloWorld_");
        assert_eq!(result, CaseType::Unknown);

        let result = CaseType::from_str("helloWorld_");
        assert_eq!(result, CaseType::Unknown);

        let result = CaseType::from_str("helloWorld_");
        assert_eq!(result, CaseType::Unknown);
    }
}
