use std::fmt;
use crate::building_block_generators::*;
use crate::setup::*;

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
    pub fn new<CT>(condition: CT, body: CodeBody) -> WhileStatement 
    where CT: CodeGenerate + 'static {
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