use crate::{case_util::CaseType, constants::TEMPLATE_VARIABLE_REGEX};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateVariable {
    Var,
    Select,
}

impl core::fmt::Display for TemplateVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateVariable::Var => write!(f, "__var__"),
            TemplateVariable::Select => write!(f, "__select__"),
        }
    }
}

impl TemplateVariable {
    pub fn is_var(&self) -> bool {
        self == &TemplateVariable::Var
    }

    pub fn is_select(&self) -> bool {
        self == &TemplateVariable::Select
    }

    pub fn from_str(value: &str) -> Option<TemplateVariable> {
        match value {
            "__var__" => Some(TemplateVariable::Var),
            "__select__" => Some(TemplateVariable::Select),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TemplateVariableInfo {
    pub template_variable: TemplateVariable,
    pub var_name: String,
    pub case_type: CaseType,
    pub start_index: usize,
    pub end_index: usize,
    pub raw_value: String,
    pub is_auto: bool,
}

impl TemplateVariableInfo {
    pub fn from_str(value: &str) -> Option<TemplateVariableInfo> {
        let is_match = TEMPLATE_VARIABLE_REGEX.is_match(value);

        if !is_match {
            return None;
        }

        let captures = TEMPLATE_VARIABLE_REGEX.captures(value).unwrap();
        let variable = captures.get(1).unwrap().as_str();
        let template_variable = TemplateVariable::from_str(variable).unwrap();

        let full = captures.get(0).unwrap();
        let second_argument = captures.get(2);
        let third_argument = captures.get(3);
        let fourth_argument = captures.get(4);

        let mut template_variable_info = TemplateVariableInfo {
            template_variable,
            var_name: "".to_string(),
            case_type: CaseType::Unknown,
            start_index: full.start(),
            end_index: full.end(),
            raw_value: full.as_str().to_string(),
            is_auto: false,
        };
        if second_argument.is_none() {
            return Some(template_variable_info);
        }

        let second_argument_str = second_argument.unwrap().as_str();

        // so condition of variables is:
        // 1. first capture have to be there always as define if select or var
        // 2. if there is second variable - it can be name or case type or auto
        // 3. if there is third variable - second can be only name and or case type, third can be only case type or auto
        // 4. if there is fourth variable - second can be only name, third can be only case type, fourth can be only auto

        if second_argument_str.starts_with("auto__") {
            template_variable_info.is_auto = true;
            return Some(template_variable_info);
        }

        if TemplateVariableInfo::is_case_type(second_argument_str) {
            template_variable_info.case_type =
                TemplateVariableInfo::get_case_type(second_argument_str);
            return Some(template_variable_info);
        }

        template_variable_info.var_name =
            second_argument_str[0..second_argument_str.len() - 2].to_string();

        if third_argument.is_none() {
            return Some(template_variable_info);
        }

        let third_argument_str = third_argument.unwrap().as_str();

        if third_argument_str.starts_with("auto__") {
            template_variable_info.is_auto = true;
            return Some(template_variable_info);
        }

        template_variable_info.case_type = TemplateVariableInfo::get_case_type(third_argument_str);

        if fourth_argument.is_none() {
            return Some(template_variable_info);
        }

        let fourth_argument_str = fourth_argument.unwrap().as_str();

        if fourth_argument_str.starts_with("auto__") {
            template_variable_info.is_auto = true;
            return Some(template_variable_info);
        }

        Some(template_variable_info)
    }

    fn is_case_type(value: &str) -> bool {
        TemplateVariableInfo::get_case_type(value) != CaseType::Unknown
    }
    fn get_case_type(value: &str) -> CaseType {
        if value.starts_with("kebab__") {
            return CaseType::KebabCase;
        }
        if value.starts_with("snake__") {
            return CaseType::SnakeCase;
        }
        if value.starts_with("camel__") {
            return CaseType::CamelCase;
        }
        if value.starts_with("pascal__") {
            return CaseType::PascalCase;
        }
        CaseType::Unknown
    }

    fn from_str_at_index(value: &str, start_index: usize) -> Option<TemplateVariableInfo> {
        let text = &value[start_index..];
        TemplateVariableInfo::from_str(text)
    }

    pub fn parse_iter(str: &str) -> TemplateVariableInfoIterator {
        TemplateVariableInfoIterator {
            content: Some(str),
            last_index: 0,
        }
    }
}

pub struct TemplateVariableInfoIterator<'a> {
    content: Option<&'a str>,
    last_index: usize,
}
impl Iterator for TemplateVariableInfoIterator<'_> {
    type Item = TemplateVariableInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.content?;
        let content = self.content.unwrap();

        let result = TemplateVariableInfo::from_str_at_index(content, self.last_index);
        if result.is_none() {
            self.content = None;
            return None;
        }
        let result = result.unwrap();

        let item_result = Some(TemplateVariableInfo {
            start_index: result.start_index + self.last_index,
            end_index: result.end_index + self.last_index,
            case_type: result.case_type,
            var_name: result.var_name,
            template_variable: result.template_variable,
            raw_value: result.raw_value,
            is_auto: result.is_auto,
        });
        self.last_index += result.end_index;
        item_result
    }
}

#[cfg(test)]
mod tests {
    use crate::{case_util::CaseType, template_variable::TemplateVariableInfo};

    use super::TemplateVariable;

    #[test]
    fn should_parse_with_iterator() {
        let search = "abc __var__ __select__ __var__1__ __select__1__ __var__2__ __select__kebab__";
        let mut iterator = TemplateVariableInfo::parse_iter(search);

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);

        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "__var__");

        let result = iterator.next().unwrap();

        assert_eq!(result.template_variable, TemplateVariable::Select);

        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "__select__");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);

        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "__var__1__");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);

        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "__select__1__");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);

        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "__var__2__");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);

        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "__select__kebab__");
    }

    #[test]
    fn should_parse_with_iterator_real_example() {
        let search = "__var__.tsx";
        let mut iterator = TemplateVariableInfo::parse_iter(search);

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "");
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        println!("{}", value_from_indexes);
        assert_eq!(value_from_indexes, "__var__");
    }

    #[test]
    fn should_test_template_variable() {
        let result = TemplateVariableInfo::from_str("__select__abc123__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::Unknown);

        let result = TemplateVariableInfo::from_str("__var__abc123__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::Unknown);

        let result = TemplateVariableInfo::from_str("__var__abc123__kebab__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::KebabCase);

        let result = TemplateVariableInfo::from_str("__var__abc123__snake__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::SnakeCase);

        let result = TemplateVariableInfo::from_str("__var__abc123__camel__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::CamelCase);

        let result = TemplateVariableInfo::from_str("__var__abc123__pascal__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::PascalCase);

        let result = TemplateVariableInfo::from_str("__var__abc123__auto__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::Unknown);
        assert!(result.is_auto);

        let result = TemplateVariableInfo::from_str("__var__abc123__kebab__auto__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "abc123");
        assert_eq!(result.case_type, CaseType::KebabCase);
        assert!(result.is_auto);

        let result = TemplateVariableInfo::from_str("__var__kebab__").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.var_name, "");
        assert_eq!(result.case_type, CaseType::KebabCase);
        assert_eq!(result.raw_value, "__var__kebab__");
    }
}
