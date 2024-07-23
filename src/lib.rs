mod building_block_generators;
mod setup;
mod flow_control_generators;
mod data_type_generators;

pub use building_block_generators::*;
pub use setup::*;
pub use flow_control_generators::*;
pub use data_type_generators::*;

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
