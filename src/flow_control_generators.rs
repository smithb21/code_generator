use std::fmt;
use crate::building_block_generators::*;
use crate::iterable::GetIter;
use crate::setup::*;

pub struct IfStatement<'a, CT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> {
    condition: CT,
    body: &'a BI,
}

impl<'a, CT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> IfStatement<'a, CT, BI> {
    pub fn new(condition: CT, body: &'a BI) -> IfStatement<'a, CT, BI> {
        IfStatement { condition, body }
    }
}

impl<'a, CT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> CodeGenerate for IfStatement<'a, CT, BI> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::If;
        let header: &[&dyn CodeGenerate;3] = &[&"if (", &self.condition, &")"];
        HeaderPlusBody::new(CodeSet::new(&header), CodeBody::new(self.body)).generate(f, info)
    }
}

pub struct WhileStatement<'a, CT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> {
    condition: CT,
    body: &'a BI,
}

impl<'a, CT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> WhileStatement<'a, CT, BI> {
    pub fn new(condition: CT, body: &'a BI) -> WhileStatement<'a, CT, BI> 
    where CT: CodeGenerate {
        WhileStatement {
            condition,
            body
        }
    }
}

impl<'a, CT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> CodeGenerate for WhileStatement<'a, CT, BI> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::While;
        let header: &[&dyn CodeGenerate;3] = &[&"while (", &self.condition, &")"];
        HeaderPlusBody::new(
            CodeSet::new(&header),
            CodeBody::new(
                self.body
            )
        ).generate(f, info)
    }
}

pub struct ForLoop<'a, IT: CodeGenerate, CT: CodeGenerate, UT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> {
    init: IT,
    continuation: CT,
    update: UT,
    body: &'a BI,
}

impl<'a, IT: CodeGenerate, CT: CodeGenerate, UT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> ForLoop<'a, IT, CT, UT, BI> {
    pub fn new(init_code: IT, continuation_code: CT, update_code: UT, body: &'a BI) -> ForLoop<'a, IT, CT, UT, BI> {
        ForLoop {
            init: init_code,
            continuation: continuation_code,
            update: update_code,
            body
        }
    }
}

impl<'a, IT: CodeGenerate, CT: CodeGenerate, UT: CodeGenerate, BI: GetIter<Item: CodeGenerate>> CodeGenerate for ForLoop<'a, IT, CT, UT, BI> {
    fn generate(&self, f: &mut fmt::Formatter<'_>, mut info: CodeGenerationInfo) -> fmt::Result {
        info.context = GeneratorContext::ForLoop;
        let header: &[&dyn CodeGenerate;7] = &[
            &"for (",
            &self.init,
            &"; ",
            &self.continuation,
            &";",
            &self.update,
            &")"
        ];
        HeaderPlusBody::new(
            CodeSet::new(&header),
            CodeBody::new(
                self.body
            )
        ).generate(f, info)
    }
}