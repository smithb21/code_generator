use core::{fmt::Display, str::Chars};
/// Performance improvement based on [https://internals.rust-lang.org/t/add-as-lowercase-et-al/15797]

use crate::CaseType;

pub const CASE_SEPARATOR: char = '`';

pub struct Cased<'a, T: ?Sized> {
    case_type: CaseType,
    source: &'a T,
}

pub trait AsCase<'a> {
    fn as_case(&'a self, case_type: CaseType) -> impl Display + 'a;
}

impl<'a> AsCase<'a> for str {
    fn as_case(&'a self, case_type: CaseType) -> impl Display + 'a {
        Cased {
            case_type,
            source: self
        }
    }
}

impl<'a> AsCase<'a> for String {
    fn as_case(&self, case_type: CaseType) -> impl Display {
        Cased {
            case_type,
            source: self
        }
    }
}

impl<'a, T: ?Sized> Cased<'a, T> {
    fn write_first_char(case_type: CaseType, char: char, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match case_type {
            CaseType::FlatCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::ScreamingCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::CamelCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::PascalCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::SnakeCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::ScreamingSnakeCase => { write!(f, "{}", char.to_uppercase()) }
        }
    }
    fn write_first_char_in_chunk(case_type: CaseType, char: char, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match case_type {
            CaseType::FlatCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::ScreamingCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::CamelCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::PascalCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::SnakeCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::ScreamingSnakeCase => { write!(f, "{}", char.to_uppercase()) }
        }
    }
    fn write_default_char(case_type: CaseType, char: char, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match case_type {
            CaseType::FlatCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::ScreamingCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::CamelCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::PascalCase => { write!(f, "{}", char.to_uppercase()) }
            CaseType::SnakeCase => { write!(f, "{}", char.to_lowercase()) }
            CaseType::ScreamingSnakeCase => { write!(f, "{}", char.to_uppercase()) }
        }
    }
    fn casify(chars: Chars<'_>, case_type: CaseType, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut result = core::fmt::Result::Ok(());

        let mut is_first_in_chunk = false;
        let mut is_first = true;

        for char in chars {
            if char == CASE_SEPARATOR {
                is_first_in_chunk = true;
                continue;
            }

            if char.is_uppercase() {
                is_first_in_chunk = true;
            }

            match (is_first, is_first_in_chunk) {
                (true, _) => {result = result.and(Self::write_first_char(case_type, char, f));}
                (_, true) => {result = result.and(Self::write_first_char_in_chunk(case_type, char, f));}
                (_, false) => {result = result.and(Self::write_default_char(case_type, char, f));}
            }

            is_first_in_chunk = false;
            is_first = false;
        }

        result
    }
}

impl<'a> Display for Cased<'a, str> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Self::casify(self.source.chars(), self.case_type, f)
    }
}

impl<'a> Display for Cased<'a, String> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Self::casify(self.source.chars(), self.case_type, f)
    }
}
