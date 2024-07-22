use std::fmt;

#[derive(Clone, Copy, Debug)]

pub enum IndentationStyle {
    Allman,
    GNU,
    Whitesmiths,
    KnR,
    Ratliff,
    Horstmann,
    Pico,
    Lisp,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum CaseType {
    FlatCase,
    ScreamingCase,
    CamelCase,
    PascalCase,
    SnakeCase,
    ScreamingSnakeCase
}

#[derive(Copy, Clone)]
pub enum IndentationType {
    Spaces,
    Tabs,
}

#[derive(Clone, Copy)]
pub enum NewLineType {
    Cr,
    Nl,
    CrNl,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum CodeStyle {
    Allman,
    GNU,
    Whitesmiths,
    KnR,
    Ratliff,
    Horstmann,
    Pico,
    Lisp,
    Minimal,
    GeneratOR,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GeneratorContext {
    If,
    While,
    ForLoop,
    Function,
    File,
    Struct,
    Enum,
    Other,
}

#[derive(Copy, Clone)]
pub struct CodeGenerationInfo {
    pub indent_level: usize,
    pub indent_type:  IndentationType,
    pub indent_amount: usize,
    pub indent_style: IndentationStyle,
    pub new_line_type: NewLineType,
    pub context: GeneratorContext,
    pub function_name_case: CaseType,
    pub member_name_case: CaseType,
    pub type_name_case: CaseType,
    pub default_name_case: CaseType,
}

pub struct DisplayHandler<'a> {
    generator: &'a dyn CodeGenerate,
    info: CodeGenerationInfo,
}

impl<'a> DisplayHandler<'a> {
    pub fn new(gen: &'a dyn CodeGenerate, info: CodeGenerationInfo) -> DisplayHandler<'a> {
        DisplayHandler { generator: gen, info: info }
    }
}

impl fmt::Display for DisplayHandler<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.generator.generate(f, self.info)
    }
}

impl CodeGenerationInfo {
    pub fn new() -> CodeGenerationInfo {
        CodeGenerationInfo {
            indent_level: 0,
            indent_amount: 4,
            indent_type: IndentationType::Spaces,
            indent_style: IndentationStyle::Allman,
            new_line_type: NewLineType::CrNl,
            context: GeneratorContext::File,
            function_name_case: CaseType::CamelCase,
            member_name_case: CaseType::SnakeCase,
            type_name_case: CaseType::PascalCase,
            default_name_case: CaseType::FlatCase,
        }
    }

    pub const fn from_style(code_style: CodeStyle) -> CodeGenerationInfo {
        match code_style {
            CodeStyle::Allman => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Allman,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::GNU => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 2,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::GNU,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::Horstmann => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Horstmann,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::KnR => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::KnR,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::Lisp => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Lisp,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::Minimal => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 0,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::None,
                new_line_type: NewLineType::None,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::Pico => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Pico,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::Ratliff => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Ratliff,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::Whitesmiths => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Whitesmiths,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
            CodeStyle::GeneratOR => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Tabs,
                indent_style: IndentationStyle::KnR,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                function_name_case: CaseType::CamelCase,
                member_name_case: CaseType::SnakeCase,
                type_name_case: CaseType::PascalCase,
                default_name_case: CaseType::FlatCase,
            },
        }
    }

    pub fn indent(&self) -> CodeGenerationInfo {
        let mut info = *self;
        info.indent_level += 1;

        info
    }

    pub fn with_context(&mut self, context: GeneratorContext) -> CodeGenerationInfo {
        let mut info = *self;
        info.context = context;

        info
    }
}

pub trait CodeGenerate {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result;
}

pub trait DisplayExt {
    fn display(self: &Self, info: CodeGenerationInfo) -> DisplayHandler;
}

impl<T> DisplayExt for T
where T: CodeGenerate {
    fn display(self: &Self, info: CodeGenerationInfo) -> DisplayHandler {
        DisplayHandler::new(self, info)
    }
}

pub struct Indentation {
}

impl Indentation {
    pub fn new() -> Indentation {
        Indentation { }
    }
}

impl CodeGenerate for Indentation {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let indent ;
        match info.indent_type {
            IndentationType::Spaces => indent = " ".repeat(info.indent_amount*info.indent_level),
            IndentationType::Tabs => indent = "	".repeat(
                ((info.indent_amount+3) / 4) * info.indent_level
            ),
        }
        
        write!(f, "{}", indent)
    }
}

pub enum NameType {
    Default,
    Type,
    Member,
    Function,
    FixedCase(CaseType),
}

pub struct Name {
    parts: Vec<String>,
    name_type: NameType
}

impl Name {
    pub fn new(screaming_snake_case_name: &str) -> Name {
        let name = screaming_snake_case_name;
        let mut parts = Vec::<String>::new();
        for entry in name.split("_") {
            if !entry.is_empty() {
                parts.push(entry.to_ascii_lowercase());
            }
        }
        if parts.len() == 0 {
            parts.push("invalid".into());
            parts.push("name".into());
        }

        Name {
            parts: parts,
            name_type: NameType::Default
        }
    }

    pub fn new_with_type(screaming_snake_case_name: &str, name_type: NameType) -> Name {
        Name::new(screaming_snake_case_name).with_type(name_type)
    }

    pub fn with_type(mut self, name_type: NameType) -> Name {
        self.name_type = name_type;

        self
    }

    fn caps_first_letter(string: String) -> String {
        let mut result = String::new();

        let mut iter = string.chars();
        if let Some(char) = iter.next() {
            result.push(char.to_ascii_uppercase());
        }

        for char in iter {
            result.push(char);
        }

        result
    }

    fn casify(&self, case: CaseType) -> String {
        let mut parts = Vec::new();
        let mut is_first = true;
        for part in self.parts.iter() {
            parts.push (match case {
                CaseType::CamelCase => if is_first {part.clone()} else {Self::caps_first_letter(part.clone())},
                CaseType::FlatCase => part.clone(),
                CaseType::PascalCase => Self::caps_first_letter(part.clone()),
                CaseType::ScreamingCase => part.to_ascii_uppercase(),
                CaseType::ScreamingSnakeCase => part.to_ascii_uppercase(),
                CaseType::SnakeCase => part.clone(),
            });
            is_first = false;
        }
        match case {
            CaseType::CamelCase => parts.join(""),
            CaseType::FlatCase => parts.join(""),
            CaseType::PascalCase => parts.join(""),
            CaseType::ScreamingCase => parts.join(""),
            CaseType::ScreamingSnakeCase => parts.join("_"),
            CaseType::SnakeCase => parts.join("_"),
        }
    }

    fn get_case_type(&self, info: CodeGenerationInfo) -> CaseType {
        match self.name_type {
            NameType::Default => info.default_name_case,
            NameType::Function => info.function_name_case,
            NameType::Member => info.member_name_case,
            NameType::Type => info.type_name_case,
            NameType::FixedCase(case) => case,
        }
    }
}

impl CodeGenerate for Name {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let case_type = self.get_case_type(info);
        write!(f, "{}", self.casify(case_type))
    }
}

pub struct NewLine {
}

impl NewLine {
    pub fn new() -> NewLine {
        NewLine { }
    }
}

impl CodeGenerate for NewLine {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        match info.new_line_type {
            NewLineType::Cr => write!(f, "\r"),
            NewLineType::Nl => write!(f, "\n"),
            NewLineType::CrNl => write!(f, "\r\n"),
            NewLineType::None => write!(f, ""),
        }
    }
}

pub struct CodeSet {
    code_set: Vec<Box<dyn CodeGenerate>>,
}

impl CodeSet {
    pub fn new(set: Vec<Box<dyn CodeGenerate>>) -> CodeSet {
        CodeSet { code_set: set }
    }
}

impl CodeGenerate for CodeSet {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result = fmt::Result::Ok(());
        let mut iter = self.code_set.iter();
        if let Some(item) = iter.next() {
            result = result.and(item.generate(f, info));

            for item in iter {
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(item.generate(f, info));
            }
        }
        result
    }
}

// No processing, just combined
pub struct JoinedCode {
    code_set: Vec<Box<dyn CodeGenerate>>,
}

impl JoinedCode {
    pub fn new(set: Vec<Box<dyn CodeGenerate>>) -> JoinedCode {
        JoinedCode { code_set: set }
    }
}

impl CodeGenerate for JoinedCode {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result = fmt::Result::Ok(());

        for item in self.code_set.iter() {
            result = result.and(item.generate(f, info));
        }

        result
    }
}

pub struct RawCode {
    text: String,
}

impl RawCode {
    pub fn new(code: &str) -> RawCode {
        RawCode { text: code.to_string() }
    }

    pub fn merge(mut self, other: RawCode) -> RawCode {
        self.text.push_str(other.text.as_str());

        self
    }
}

impl CodeGenerate for RawCode {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        // First line doesn't print indentation
        if let Some(line) = self.text.lines().next() {
            result = result.and(result.and(write!(f, "{}", line)));
            
            for line in self.text.lines().skip(1) {
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "{}", line));
            }
        }
        return result;
    }
}

impl CodeGenerate for String {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        // First line doesn't print indentation
        if let Some(line) = self.lines().next() {
            result = result.and(result.and(write!(f, "{}", line)));
            
            for line in self.lines().skip(1) {
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "{}", line));
            }
        }
        return result;
    }
}

pub struct SeparatedCode {
    items: Vec<Box<dyn CodeGenerate>>,
    separator: Box<dyn CodeGenerate>,
    // TODO: newlines in GeneratorInfo?
}

impl SeparatedCode {
    pub fn new(items: Vec<Box<dyn CodeGenerate>>, separator: Box<dyn CodeGenerate>) -> SeparatedCode {
        SeparatedCode { items: items, separator: separator }
    }
}

impl CodeGenerate for SeparatedCode {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());

        let mut iterator = self.items.iter();

        if let Some(item) = iterator.next() {
            result = result.and(item.generate(f, info));
        }

        for item in iterator {
            result = result.and(self.separator.generate(f, info));
            result = result.and(item.generate(f, info));
        }

        result
    }
}

pub struct CodeBody {
    raw_code: CodeSet,
}

impl CodeBody {
    pub fn new(code: Vec<Box<dyn CodeGenerate>>) -> CodeBody {
        CodeBody {raw_code: CodeSet::new(code)}
    }
}

impl CodeGenerate for CodeBody {
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        match info.indent_style {
            IndentationStyle::Allman => {
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "{{"));
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info.indent()));
                result = result.and(self.raw_code.generate(f, info.indent()));
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "}}"));
            }
            IndentationStyle::GNU => {
                result = result.and(write!(f, "{{"));
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info.indent().indent()));
                result = result.and(self.raw_code.generate(f, info.indent().indent()));
                result = result.and(NewLine::new().generate(f, info));
                if info.context != GeneratorContext::Function {
                    result = result.and(Indentation::new().generate(f, info.indent()));
                }
                result = result.and(write!(f, "}}"));
            }
            IndentationStyle::Horstmann => {
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "{{"));
                let mut temp_info = info;
                temp_info.indent_level = 1;
                temp_info.indent_amount -= 1;// to account for '{' if using spaces
                result = result.and(Indentation::new().generate(f, temp_info));
                result = result.and(self.raw_code.generate(f, info.indent()));
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "}}"));
            }
            IndentationStyle::KnR => {
                result = result.and(write!(f, "{{"));
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info.indent()));
                result = result.and(self.raw_code.generate(f, info.indent()));
                result = result.and(NewLine::new().generate(f, info));
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "}}"));
            }
            IndentationStyle::Pico => {
                result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, "{{"));
                let mut temp_info = info;
                temp_info.indent_level = 1;
                temp_info.indent_amount -= 1;// to account for '{' if using spaces
                result = result.and(Indentation::new().generate(f, temp_info));
                result = result.and(self.raw_code.generate(f, info.indent()));
                //result = result.and(Indentation::new().generate(f, info));
                result = result.and(write!(f, " }}"));
            }
            IndentationStyle::None => {
                result = result.and(write!(f, "{{"));
                result = result.and(self.raw_code.generate(f, info.indent()));
                result = result.and(write!(f, "}}"));
            }
            _ => result = result.and(write!(f, "NOT SUPPORTED YET!")),
        }

        result
    }
}

pub struct HeaderPlusBody<HT> {
    header: HT,
    body: CodeBody,
}

impl<HT> HeaderPlusBody<HT> {
    pub fn new(header: HT, body: CodeBody) -> HeaderPlusBody<HT>{
        HeaderPlusBody {header: header, body: body}
    }
}

impl<HT> CodeGenerate for HeaderPlusBody<HT>
where HT: CodeGenerate,{
    fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
        let mut result: fmt::Result = fmt::Result::Ok(());
        result = result.and(self.header.generate(f, info));
        match info.indent_style {
            IndentationStyle::Allman |
            IndentationStyle::Horstmann |
            IndentationStyle::Pico => {
                result = result.and(NewLine::new().generate(f, info));
            },
            IndentationStyle::GNU => {
                result = result.and(NewLine::new().generate(f, info));
                if info.context != GeneratorContext::Function {
                    result = result.and(Indentation::new().generate(f, info.indent()));
                }
            }
            IndentationStyle::KnR => {
                result = result.and(write!(f, " "));
            }
            _ => (),
        }
        result = result.and(self.body.generate(f, info));
        result
    }
}

pub struct IfStatement {
    content: HeaderPlusBody<JoinedCode>,
}

impl IfStatement {
    pub fn new<CT>(condition: CT, body: CodeBody) -> IfStatement
    where CT: CodeGenerate + 'static {
        IfStatement {
            content: HeaderPlusBody::new(
                JoinedCode::new(
                    vec![
                        Box::new(String::from("if (")),
                        Box::new(condition),
                        Box::new(String::from(")"))
                    ]
                ),
                body
            )
        }
    }
}

impl CodeGenerate for IfStatement {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::If;
        self.content.generate(f, info)
    }
}

pub struct WhileStatement {
    content: HeaderPlusBody<JoinedCode>,
}

impl WhileStatement {
    pub fn new(condition: RawCode, body: CodeBody) -> WhileStatement {
        WhileStatement {
            content: HeaderPlusBody::new(
                JoinedCode::new(
                    vec![
                        Box::new(String::from("while (")),
                        Box::new(condition),
                        Box::new(String::from(")"))
                    ]
                ),
                body
            )
        }
    }
}

impl CodeGenerate for WhileStatement {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::While;
        self.content.generate(f, info)
    }
}

pub struct ForLoop {
    content: HeaderPlusBody<JoinedCode>,
}

impl ForLoop {
    pub fn new<IT, CT, UT>(init_code: IT, continuation_code: CT, update_code: UT, body: Vec<Box<dyn CodeGenerate>>) -> ForLoop
    where IT: CodeGenerate + 'static,
        CT: CodeGenerate + 'static,
        UT: CodeGenerate + 'static {
        ForLoop { content: HeaderPlusBody::new(
            JoinedCode::new(vec![
                Box::new(String::from("for (")),
                Box::new(init_code),
                Box::new(String::from("; ")),
                Box::new(continuation_code),
                Box::new(String::from("; ")),
                Box::new(update_code),
                Box::new(String::from(")")),
            ]),
            CodeBody::new(body)
        ) }
    }
}

impl CodeGenerate for ForLoop {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::ForLoop;
        self.content.generate(f, info)
    }
}

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