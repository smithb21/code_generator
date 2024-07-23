use std::fmt;
use crate::building_block_generators::*;
use crate::setup::*;

pub struct FunctionSignature {
    function_name: Name,
    parameters: SeparatedCode,
    return_type: Name,
}

impl FunctionSignature {
    pub fn new(return_type: Name, name: Name, parameters: Vec<(Name, Name)>) -> FunctionSignature {
        let mut set = Vec::<Box<dyn CodeGenerate>>::new();
        for (type_name, param_name) in parameters {
            set.push(Box::new(JoinedCode::new(vec![Box::new(type_name.with_type(NameType::Type)), Box::new(String::from(" ")), Box::new(param_name.with_type(NameType::Member))])))
        }

        FunctionSignature {
            return_type: return_type.with_type(NameType::Type),
            function_name: name.with_type(NameType::Function),
            parameters: SeparatedCode::new(set, Box::new(String::from(", "))),
        }
    }
}

impl CodeGenerate for FunctionSignature {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        result = result.and(self.return_type.generate(f, info));
        result = result.and(write!(f, " "));
        result = result.and(self.function_name.generate(f, info));
        result = result.and(write!(f, "("));
        result = result.and(self.parameters.generate(f, info));
        result = result.and(write!(f, ")"));

        result
    }
}

pub struct FunctionDeclaration {
    signature: FunctionSignature
}

impl FunctionDeclaration {
    pub fn new(return_type: Name, name: Name, parameters: Vec<(Name, Name)>) -> FunctionDeclaration {
        FunctionDeclaration {
            signature: FunctionSignature::new(return_type, name, parameters)
        }
    }
}

impl CodeGenerate for FunctionDeclaration {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        result = result.and(self.signature.generate(f, info));
        result = result.and(write!(f, ";"));

        result
    }
}

pub struct Function {
    content: HeaderPlusBody<FunctionSignature>,
}

impl Function {
    pub fn new(signature: FunctionSignature, body: CodeBody) -> Function {
        Function {
            content: HeaderPlusBody::new(signature, body)
        }
    }
}

impl CodeGenerate for Function {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Function;
        self.content.generate(f, info)
    }
}

pub struct HeaderFile {
    file_name: Name,
    content: CodeSet,
}

impl HeaderFile {
    pub fn new(file_name: Name, content: CodeSet) -> HeaderFile {
        HeaderFile {
            file_name: file_name.with_type(NameType::FixedCase(CaseType::ScreamingSnakeCase)),
            content: content,
        }
    }
}

impl CodeGenerate for HeaderFile {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::File;

        let mut result = fmt::Result::Ok(());

        result = result.and(write!(f, "#ifndef {}_H", self.file_name.display(info)));
        result = result.and(NewLine::new().generate(f, info));
        result = result.and(write!(f, "#define {}_H", self.file_name.display(info)));
        result = result.and(NewLine::new().generate(f, info));
        result = result.and(NewLine::new().generate(f, info));

        result = result.and(self.content.generate(f, info));
        result = result.and(NewLine::new().generate(f, info));
        result = result.and(NewLine::new().generate(f, info));

        result = result.and(write!(f, "#endif"));
        result = result.and(NewLine::new().generate(f, info));

        result
    }
}

pub struct Enum {
    content: HeaderPlusBody<String>,
    name: Name,
}

impl Enum {
    pub fn new(name: Name, values: Vec<(Name, Option<i64>)>) -> Enum {
        let mut code_values: Vec<Box<dyn CodeGenerate>> = Vec::new();
        for (member_name, value) in values {
            if let Some(value) = value {
                code_values.push(Box::new(JoinedCode::new(
                    vec![Box::new(member_name.with_type(NameType::Type)), Box::new(format!(" = {},", value))]
                )));
            } else {
                code_values.push(Box::new(JoinedCode::new(
                    vec![Box::new(member_name.with_type(NameType::Type)), Box::new(String::from(","))]
                )));
            }
        }
        Enum {
            name: name.with_type(NameType::Type),
            content: HeaderPlusBody::new(
                String::from("typedef enum"),
                CodeBody::new(code_values)
            )
        }
    }
}

impl CodeGenerate for Enum {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Enum;
        let mut result = self.content.generate(f, info);
        result = result.and(String::from(" ").generate(f, info));
        result = result.and(self.name.generate(f, info));
        result = result.and(String::from(";").generate(f, info));

        result
    }
}

pub struct Struct {
    content: HeaderPlusBody<String>,
    name: Name,
}

impl Struct {
    pub fn new(name: Name, values: Vec<(Name, Name)>) -> Struct {
        let mut code_values: Vec<Box<dyn CodeGenerate>> = Vec::new();
        for (member_type, member_name) in values {
            code_values.push(Box::new(JoinedCode::new(vec![
                Box::new(member_type.with_type(NameType::Type)),
                Box::new(String::from(" ")),
                Box::new(member_name.with_type(NameType::Member)),
                Box::new(String::from(";")),
            ])));
        }
        Struct {
            name: name.with_type(NameType::Type),
            content: HeaderPlusBody::new(
                String::from("typedef struct"),
                CodeBody::new(code_values)
            )
        }
    }
}

impl CodeGenerate for Struct {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Struct;
        let mut result = self.content.generate(f, info);
        result = result.and(String::from(" ").generate(f, info));
        result = result.and(self.name.generate(f, info));
        result = result.and(String::from(";").generate(f, info));

        result
    }
}

pub struct TypeDef {
    defined_type: String,
    name: Name,
}

impl TypeDef {
    pub fn new(name: Name, defined_type: String) -> TypeDef {
        TypeDef { 
            defined_type: defined_type,
            name: name.with_type(NameType::Type)
        }
    }
}

impl CodeGenerate for TypeDef {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Struct;
        let mut result = String::from("typedef ").generate(f, info);
        result = result.and(self.defined_type.generate(f, info));
        result = result.and(String::from(" ").generate(f, info));
        result = result.and(self.name.generate(f, info));
        result = result.and(String::from(";").generate(f, info));

        result
    }
}