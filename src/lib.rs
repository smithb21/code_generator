//! This generator is designed to allow easy reuse of code generation
//! 
//! # Example
//! 
//! This example shows how the same structure can be generated into two
//! different formats.
//! 
//! ```
//! # use code_generator::CodeStyle;
//! # use code_generator::DisplayExt;
//! # use code_generator::CodeGenerationInfo;
//! # use code_generator::FunctionSignature;
//! # use code_generator::Name;
//! # use code_generator::NewLineType;
//! # use code_generator::CodeBody;
//! # use code_generator::IfStatement;
//! # use code_generator::NameType;
//! # use code_generator::ForLoop;
//! # use code_generator::Function;
//! # use code_generator::CodeSet;
//! #
//! let code = CodeSet::new(vec![
//!     Box::new(String::from("#include \"test.h\"")),
//!     Box::new(String::from("")),
//!     Box::new(String::from("")),
//!     Box::new(Function::new(FunctionSignature::new(
//!             Name::new("MY_RETURN_TYPE_ONE"),
//!             Name::new("MY_FUNCTION_NAME_ONE"),
//!             vec![
//!                 (Name::new("PARAM_TYPE_ONE"), Name::new("PARAM_NAME_ONE")),
//!                 (Name::new("PARAM_TYPE_TWO"), Name::new("PARAM_NAME_TWO")),
//!                 (Name::new("PARAM_TYPE_THREE"), Name::new("PARAM_NAME_THREE")),
//!             ]
//!         ),
//!         CodeBody::new(vec![
//!             Box::new(IfStatement::new(
//!                     Name::new_with_type("PARAM_NAME_THREE", NameType::Member),
//!                 CodeBody::new(vec![
//!                     Box::new(ForLoop::new(
//!                         String::from("ParamTypeTwo i = 0"),
//!                         String::from("i < param_name_two"),
//!                         String::from("i++"),
//!                         vec![
//!                             Box::new(String::from("printf(\"ParamOne: {}\\n\", param_name_one);"))
//!                         ]
//!                     ))
//!                 ])
//!             ))
//!         ])
//!     )),
//!     Box::new(String::from("")),
//! ]);
//! 
//! let mut info = CodeGenerationInfo::from_style(CodeStyle::KnR);
//! info.set_new_line_type(NewLineType::Nl);
//! assert_eq!(
//! "#include \"test.h\"
//!
//!
//! MyReturnTypeOne my_function_name_one(ParamTypeOne param_name_one, ParamTypeTwo param_name_two, ParamTypeThree param_name_three) {
//!     if (param_name_three) {
//!         for (ParamTypeTwo i = 0; i < param_name_two; i++) {
//!             printf(\"ParamOne: {}\\n\", param_name_one);
//!         }
//!     }
//! }
//! ", format!("{}", code.display(info)));
//!
//! let mut info = CodeGenerationInfo::from_style(CodeStyle::Allman);
//! info.set_new_line_type(NewLineType::Nl);
//! assert_eq!(
//! "#include \"test.h\"
//!
//!
//! MyReturnTypeOne my_function_name_one(ParamTypeOne param_name_one, ParamTypeTwo param_name_two, ParamTypeThree param_name_three)
//! {
//!     if (param_name_three)
//!     {
//!         for (ParamTypeTwo i = 0; i < param_name_two; i++)
//!         {
//!             printf(\"ParamOne: {}\\n\", param_name_one);
//!         }
//!     }
//! }
//! ", format!("{}", code.display(info)));
//!
//!
//!
//! ```
//! 
//! Because the output format is not dependent on the input data structure, it
//! is very easy to make code generation modular. Any pieces of generated code
//! that share structure, can share a generator.

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

    #[test]
    fn name_camel_case() {
        let default_gen_info: CodeGenerationInfo = CodeGenerationInfo::from_style(CodeStyle::KnR);
        let case = NameType::FixedCase(CaseType::CamelCase);
        let name = Name::new_with_type("TEST_NAME", case);
        let result = format!("{}", name.display(default_gen_info));
        assert_eq!(result, "testName");
    }
}
