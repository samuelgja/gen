use std::path::Display;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::map::Iter;

use crate::{
    case_util::CaseType,
    constants::{TEMPLATE_SELECT_REGEX, TEMPLATE_VARIABLE_REGEX},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateVariable {
    Var,
    Select,
}

impl core::fmt::Display for TemplateVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateVariable::Var => write!(f, "#var"),
            TemplateVariable::Select => write!(f, "#select"),
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TemplateVariableParser {
    pub template_variable: TemplateVariable,
    pub index: usize,
    pub case_type: CaseType,
    pub start_index: usize,
    pub end_index: usize,
}

impl TemplateVariable {
    pub fn from_str(value: &str) -> Option<TemplateVariableParser> {
        let is_var_match = TEMPLATE_VARIABLE_REGEX.is_match(value);
        let is_select_match = TEMPLATE_SELECT_REGEX.is_match(value);

        let template_variable = if is_var_match && is_select_match {
            let var_capture = TEMPLATE_VARIABLE_REGEX.captures(value).unwrap();
            let select_capture = TEMPLATE_SELECT_REGEX.captures(value).unwrap();
            let var_start_index = var_capture.get(0).unwrap().start();
            let select_start_index = select_capture.get(0).unwrap().start();
            if var_start_index < select_start_index {
                TemplateVariable::Var
            } else {
                TemplateVariable::Select
            }
        } else if is_var_match {
            TemplateVariable::Var
        } else if is_select_match {
            TemplateVariable::Select
        } else {
            return None;
        };

        let mut end_index = 0;
        let mut start_index = 0;

        let is_var_match = template_variable.is_var();
        let is_select_match = template_variable.is_select();
        let index = if is_var_match {
            let captures = TEMPLATE_VARIABLE_REGEX.captures(value).unwrap();
            let var_capture = captures.get(0).unwrap();
            start_index = var_capture.start();
            if let Some(index) = captures.get(1) {
                end_index = index.end();
                index.as_str().parse::<usize>().unwrap()
            } else {
                end_index = var_capture.end();
                0
            }
        } else if is_select_match {
            let captures = TEMPLATE_SELECT_REGEX.captures(value).unwrap();
            let var_capture = captures.get(0).unwrap();
            start_index = var_capture.start();
            if let Some(index) = captures.get(1) {
                end_index = index.end();
                index.as_str().parse::<usize>().unwrap()
            } else {
                end_index = captures.get(0).unwrap().end();
                0
            }
        } else {
            0
        };

        // suffix can be _pascal, _camel, _kebab, _snake
        let suffix = if value.len() > end_index {
            &value[end_index..]
        } else {
            ""
        };

        let case_type = if suffix.starts_with("_pascal") {
            end_index += 7;
            CaseType::PascalCase
        } else if suffix.starts_with("_camel") {
            end_index += 6;
            CaseType::CamelCase
        } else if suffix.starts_with("_kebab") {
            end_index += 6;
            CaseType::KebabCase
        } else if suffix.starts_with("_snake") {
            end_index += 6;
            CaseType::SnakeCase
        } else {
            CaseType::Unknown
        };

        Some(TemplateVariableParser {
            template_variable,
            index,
            case_type,
            end_index,
            start_index,
        })
    }

    fn from_str_at_index(value: &str, start_index: usize) -> Option<TemplateVariableParser> {
        let text = &value[start_index..];
        return TemplateVariable::from_str(&text);
    }

    pub fn parse_iter<'a>(str: &'a str) -> TemplateVarIterator {
        TemplateVarIterator {
            content: Some(str),
            last_index: 0,
        }
    }
}

pub struct TemplateVarIterator<'a> {
    content: Option<&'a str>,
    last_index: usize,
}
impl Iterator for TemplateVarIterator<'_> {
    type Item = TemplateVariableParser;

    fn next(&mut self) -> Option<Self::Item> {
        if self.content.is_none() {
            return None;
        }
        let content = self.content.unwrap();

        let result = TemplateVariable::from_str_at_index(content, self.last_index);
        if result.is_none() {
            self.content = None;
            return None;
        }
        let result = result.unwrap();

        let item_result = Some(TemplateVariableParser {
            start_index: result.start_index + self.last_index,
            end_index: result.end_index + self.last_index,
            case_type: result.case_type,
            index: result.index,
            template_variable: result.template_variable,
        });
        self.last_index += result.end_index;
        return item_result;
    }
}

#[cfg(test)]
mod tests {
    use super::TemplateVariable;

    #[test]
    fn should_parse_with_iterator() {
        let search = "abc #var #select #var1 #select1 #var2 #select_kebab";
        let mut iterator = TemplateVariable::parse_iter(search);

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 0);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#var");

        let result = iterator.next().unwrap();

        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.index, 0);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#select");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 1);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#var1");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.index, 1);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#select1");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 2);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#var2");

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.index, 0);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#select_kebab");
    }

    #[test]
    fn should_parse_with_iterator_real_example() {
        let search = "#var.tsx";
        let mut iterator = TemplateVariable::parse_iter(search);

        let result = iterator.next().unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 0);
        let value_from_indexes = search[result.start_index..result.end_index].to_owned();
        assert_eq!(value_from_indexes, "#var");
    }

    #[test]
    fn should_test_template_variable() {
        let result = TemplateVariable::from_str("abc #var #select").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 0);

        let result = TemplateVariable::from_str("##var.tsx").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 0);

        let result = TemplateVariable::from_str("#var2").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Var);
        assert_eq!(result.index, 2);

        let result = TemplateVariable::from_str("#select").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.index, 0);

        let result = TemplateVariable::from_str("#select=").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.index, 0);

        let result = TemplateVariable::from_str("#select1$asd").unwrap();
        assert_eq!(result.template_variable, TemplateVariable::Select);
        assert_eq!(result.index, 1);

        let result = TemplateVariable::from_str("$te");
        assert_eq!(result.is_none(), true);

        let result = TemplateVariable::from_str("#var_pascal");
        assert_eq!(
            result.unwrap().case_type,
            crate::case_util::CaseType::PascalCase
        );

        let result = TemplateVariable::from_str("#var_camel");
        assert_eq!(
            result.unwrap().case_type,
            crate::case_util::CaseType::CamelCase
        );

        let result = TemplateVariable::from_str("#var_kebab");
        assert_eq!(
            result.unwrap().case_type,
            crate::case_util::CaseType::KebabCase
        );

        let result = TemplateVariable::from_str("#var_snake");
        assert_eq!(
            result.unwrap().case_type,
            crate::case_util::CaseType::SnakeCase
        );

        let result = TemplateVariable::from_str("#var1_snake");
        assert_eq!(
            result.unwrap().case_type,
            crate::case_util::CaseType::SnakeCase
        );
    }
}
