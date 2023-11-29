use convert_case::{Case, Casing};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CaseType {
    SnakeCase,
    KebabCase,
    CamelCase,
    PascalCase,
    Unknown,
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

        return CaseType::Unknown;
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
