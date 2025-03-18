use std::fmt;
use crate::building_block_generators::*;
use crate::iterable::GetIter;
use crate::setup::*;

struct Param<'a> {
    p_type: Name<'a>,
    name: Name<'a>,
}
impl<'a> CodeGenerate for Param<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        write!(f, "{} {}", self.p_type.display(info), self.name.display(info))
    }
}

pub struct FunctionSignature<'a> {
    function_name: Name<'a>,
    parameters: &'a [(Name<'a>, Name<'a>)],
    return_type: Name<'a>,
}

impl<'a> FunctionSignature<'a> {
    pub fn new(return_type: Name<'a>, name: Name<'a>, parameters: &'a [(Name<'a>, Name<'a>)]) -> FunctionSignature<'a> {
        FunctionSignature {
            return_type: return_type.with_type(NameType::Type),
            function_name: name.with_type(NameType::Function),
            parameters: parameters,
        }
    }
}

impl<'a> CodeGenerate for FunctionSignature<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        result = result.and(self.return_type.generate(f, info));
        result = result.and(write!(f, " "));
        result = result.and(self.function_name.generate(f, info));
        result = result.and(write!(f, "("));
        result = result.and(SeparatedCode::new(&self.parameters.iter().map(|(a, b)| Param { p_type: *a, name: *b }), &", ").generate(f, info));
        result = result.and(write!(f, ")"));

        result
    }
}

pub struct FunctionDeclaration<'a> {
    signature: FunctionSignature<'a>
}

impl<'a> FunctionDeclaration<'a> {
    pub fn new(return_type: Name<'a>, name: Name<'a>, parameters: &'a [(Name<'a>, Name<'a>)]) -> FunctionDeclaration<'a> {
        FunctionDeclaration {
            signature: FunctionSignature::new(return_type, name, parameters)
        }
    }
}

impl<'a> CodeGenerate for FunctionDeclaration<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        result = result.and(self.signature.generate(f, info));
        result = result.and(write!(f, ";"));

        result
    }
}

impl<'a> From<FunctionSignature<'a>> for FunctionDeclaration<'a> {
    fn from(value: FunctionSignature<'a>) -> Self {
        Self { signature: value }
    }
}

pub struct Function<'a, I: CodeGenerate, T: GetIter<Item=I>> {
    content: HeaderPlusBody<'a, FunctionSignature<'a>, T>,
}

impl<'a, I: CodeGenerate, T: GetIter<Item=I>> Function<'a, I, T> {
    pub fn new(signature: FunctionSignature<'a>, body: &'a T) -> Function<'a, I, T> {
        Function {
            content: HeaderPlusBody::new(signature, CodeBody::new(body))
        }
    }
}

impl<'a, I: CodeGenerate, T: GetIter<Item=I>> CodeGenerate for Function<'a, I, T> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Function;
        self.content.generate(f, info)
    }
}

pub struct FunctionCall<'a>  {
    name: Name<'a>,
    params: &'a [(Name<'a>, Name<'a>)],
    is_terminated: bool,
}

impl<'a> FunctionCall<'a> {
    pub fn new(name: Name<'a>, params: &'a [(Name<'a>, Name<'a>)]) -> FunctionCall<'a> {
        FunctionCall {
            name: name.with_type(NameType::Function),
            params,
            is_terminated: false
        }
    }

    pub fn new_with_end(name: Name<'a>, params: &'a [(Name<'a>, Name<'a>)]) -> FunctionCall<'a> {
        FunctionCall {
            name: name.with_type(NameType::Function),
            params,
            is_terminated: true
        }
    }
}

impl<'a> CodeGenerate for FunctionCall<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let termination = if self.is_terminated {";"} else {""};
        write!(f, "{}({}){}", self.name.display(info), SeparatedCode::new(&self.params.iter().map(|(a, b)| Param {p_type: *a, name: *b}), &", ").display(info), termination)
    }
}

pub struct HeaderFile<'a, T: GetIter<Item: CodeGenerate>> {
    file_name: Name<'a>,
    content: CodeSet<'a, T>,
}

impl<'a, T: GetIter<Item: CodeGenerate>> HeaderFile<'a, T> {
    pub fn new(file_name: Name<'a>, content: CodeSet<'a, T>) -> HeaderFile<'a, T> {
        HeaderFile {
            file_name: file_name.with_type(NameType::ConstDefine),
            content: content,
        }
    }
}

impl<'a, T: GetIter<Item: CodeGenerate>> CodeGenerate for HeaderFile<'a, T> {
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

struct EnumEntry<'a> {
    name: Name<'a>,
    value: Option<i128>
}

impl<'a> CodeGenerate for EnumEntry<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        match self.value {
            Some(value) => { write!(f, "{} = {},", self.name.display(info), value) }
            None => { write!(f, "{},", self.name.display(info)) }
        }
    }
}

pub struct Enum<'a> {
    content: &'a[(Name<'a>, Option<i128>)],
    name: Name<'a>,
}

impl<'a> Enum<'a> {
    pub fn new(name: Name<'a>, values: &'a[(Name<'a>, Option<i128>)]) -> Enum<'a> {
        Enum {
            name: name.with_type(NameType::Type),
            content: values
        }
    }
}

impl<'a> CodeGenerate for Enum<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Enum;
        let mut result = HeaderPlusBody::new(
            "typedef enum",
            CodeBody::new(&self.content.iter().map(
                |(a,b)|
                EnumEntry {
                    name: a.with_type(NameType::ConstDefine), value: *b
                }))
        ).generate(f, info);
        result = result.and(" ".generate(f, info));
        result = result.and(self.name.generate(f, info));
        result = result.and(";".generate(f, info));

        result
    }
}



struct StructEntry<'a> {
    e_type: Name<'a>,
    name: Name<'a>
}

impl<'a> CodeGenerate for StructEntry<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        write!(f, "{} = {},", self.e_type.display(info), self.name.display(info))
    }
}

pub struct Struct<'a> {
    content: &'a [(Name<'a>, Name<'a>)],
    name: Name<'a>,
}

impl<'a> Struct<'a> {
    pub fn new(name: Name<'a>, values: &'a [(Name<'a>, Name<'a>)]) -> Struct<'a> {
        Struct {
            name: name.with_type(NameType::Type),
            content: values
        }
    }
}

impl<'a> CodeGenerate for Struct<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::Struct;
        let mut result = HeaderPlusBody::new(
            "typedef struct",
             CodeBody::new(&self.content.iter().map(
                |(a, b)|
                StructEntry {
                    e_type: a.with_type(NameType::Type),
                    name: b.with_type(NameType::Member)
                }))
        ).generate(f, info);
        result = result.and(" ".generate(f, info));
        result = result.and(self.name.generate(f, info));
        result = result.and(";".generate(f, info));

        result
    }
}

pub struct TypeDef<'a> {
    defined_type: Name<'a>,
    name: Name<'a>,
}

impl<'a> TypeDef<'a> {
    pub fn new(name: Name<'a>, defined_type: Name<'a>) -> TypeDef<'a> {
        TypeDef {
            defined_type: defined_type.with_type(NameType::Type),
            name: name.with_type(NameType::Type)
        }
    }
}

impl<'a> CodeGenerate for TypeDef<'a> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result = "typedef ".generate(f, info);
        result = result.and(self.defined_type.generate(f, info));
        result = result.and(" ".generate(f, info));
        result = result.and(self.name.generate(f, info));
        result = result.and(";".generate(f, info));


        result
    }
}


pub struct ConstDefine<'a, VT> {
    name: Name<'a>,
    value: VT,
}

impl<'a, VT> ConstDefine<'a, VT> {
    /// Creates a ConstDefine generator
    /// 
    /// ```
    /// # use code_generator::CodeGenerationInfo;
    /// # use code_generator::DisplayExt;
    /// # use code_generator::JoinedCode;
    /// # use code_generator::Name;
    /// # use code_generator::ConstDefine;
    /// #
    /// let define = ConstDefine::new(Name::new("name"), String::from("2"));
    /// let mut info = CodeGenerationInfo::new();
    /// assert_eq!("#define NAME 2", format!("{}", define.display(info)));
    /// ```
    pub fn new(name: Name, value: VT) -> ConstDefine<VT> {
        ConstDefine { name: name.with_type(NameType::ConstDefine), value }
    }
}

impl<'a, VT> CodeGenerate for ConstDefine<'a, VT>
where VT: CodeGenerate + 'static {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result = "#define ".generate(f, info);
        result = result.and(self.name.generate(f, info));
        result = result.and(" ".generate(f, info));
        result = result.and(self.value.generate(f, info));

        result
    }
}
