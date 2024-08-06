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
    ScreamingSnakeCase,
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
    Default,
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
pub struct CaseTypes {
    pub const_define_case: CaseType,
    pub function_name_case: CaseType,
    pub member_name_case: CaseType,
    pub type_name_case: CaseType,
    pub file_name_case: CaseType,
    pub default_case: CaseType,
}

impl CaseTypes {
    pub fn new() -> CaseTypes {
        CaseTypes {
            const_define_case: CaseType::ScreamingSnakeCase,
            function_name_case: CaseType::SnakeCase,
            member_name_case: CaseType::SnakeCase,
            type_name_case: CaseType::PascalCase,
            file_name_case: CaseType::PascalCase,
            default_case: CaseType::SnakeCase
        }
    }

    pub fn with_const_define(mut self, case_type: CaseType) -> Self {
        self.const_define_case = case_type;
        self
    }

    pub fn with_function_name(mut self, case_type: CaseType) -> Self {
        self.function_name_case = case_type;
        self
    }

    pub fn with_member_name(mut self, case_type: CaseType) -> Self {
        self.member_name_case = case_type;
        self
    }

    pub fn with_type_name(mut self, case_type: CaseType) -> Self {
        self.type_name_case = case_type;
        self
    }

    pub fn with_file_name(mut self, case_type: CaseType) -> Self {
        self.file_name_case = case_type;
        self
    }

    pub fn with_default(mut self, case_type: CaseType) -> Self {
        self.default_case = case_type;
        self
    }
}

#[derive(Copy, Clone)]
pub struct CodeGenerationInfo {
    pub indent_level: usize,
    pub indent_type:  IndentationType,
    pub indent_amount: usize,
    pub indent_style: IndentationStyle,
    pub new_line_type: NewLineType,
    pub context: GeneratorContext,
    pub case_types: CaseTypes,
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
            case_types: CaseTypes::new(),
        }
    }

    pub fn with_ident_style(mut self, style: IndentationStyle) -> Self {
        self.indent_style = style;
        self
    }

    pub fn with_ident_amount(mut self, amount: usize) -> Self {
        self.indent_amount = amount;
        self
    }

    pub fn with_indent_type(mut self, indent_type: IndentationType) -> Self {
        self.indent_type = indent_type;
        self
    }

    pub fn with_new_line_type(mut self, new_line_type: NewLineType) -> Self {
        self.new_line_type = new_line_type;
        self
    }

    pub fn with_case_types(mut self, case_types: CaseTypes) -> Self {
        self.case_types = case_types;
        self
    }

    pub fn from_style(code_style: CodeStyle) -> CodeGenerationInfo {
        match code_style {
            CodeStyle::Allman => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Allman,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::GNU => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 2,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::GNU,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Horstmann => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Horstmann,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::KnR => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::KnR,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Lisp => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Lisp,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Minimal => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 0,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::None,
                new_line_type: NewLineType::None,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Pico => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Pico,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Ratliff => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Ratliff,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Whitesmiths => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Spaces,
                indent_style: IndentationStyle::Whitesmiths,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
            },
            CodeStyle::Default => CodeGenerationInfo {
                indent_level: 0,
                indent_amount: 4,
                indent_type: IndentationType::Tabs,
                indent_style: IndentationStyle::KnR,
                new_line_type: NewLineType::CrNl,
                context: GeneratorContext::File,
                case_types: CaseTypes::new(),
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

    pub fn set_new_line_type(&mut self, new_line_type: NewLineType) {
        self.new_line_type = new_line_type;
    }
}

pub trait CodeGenerate {
    /// Trait function which allows the generation of code with context
    /// 
    /// # Example Implementation
    ///
    /// ```
    /// # use code_generator::CodeStyle;
    /// # use code_generator::CodeGenerationInfo;
    /// # use code_generator::Indentation;
    /// # use code_generator::DisplayExt;
    /// # use code_generator::CodeGenerate;
    /// # use std::fmt;
    /// #
    /// struct Example {
    ///     a: u32,
    /// }
    /// 
    /// impl CodeGenerate for Example {
    ///     fn generate(&self, f: &mut fmt::Formatter<'_>, info: CodeGenerationInfo) -> fmt::Result {
    ///         write!(f, "{}", self.a)
    ///     }
    /// }
    /// 
    /// let example = Example {a: 69};
    /// let info = CodeGenerationInfo::from_style(CodeStyle::KnR);
    /// assert_eq!("69", format!("{}", example.display(info)));
    /// ```
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