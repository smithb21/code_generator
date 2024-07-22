mod c_generator;

pub use c_generator::*;

#[cfg(test)]
mod tests {
    use super::*;

    static DEFAULT_GEN_INFO: CodeGenerationInfo = CodeGenerationInfo::from_style(CodeStyle::KnR);

    #[test]
    fn name_camel_case() {
        let case = NameType::FixedCase(CaseType::CamelCase);
        let name = Name::new_with_type("TEST_NAME", case);
        let result = format!("{}", name.display(DEFAULT_GEN_INFO));
        assert_eq!(result, "testName");
    }
}
