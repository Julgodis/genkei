use crate::StyleError;
use std::fmt::Write;

/// Media queries for responsive design
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MediaQuery {
    /// Breakpoint: sm 640px
    Sm,
    /// Breakpoint: md 768px
    Md,
    /// Breakpoint: lg 1024px
    Lg,
    /// Breakpoint: xl 1280px
    Xl,
    /// Breakpoint: 2xl 1536px
    Xxl,
    /// Breakpoint: 3xl 1920px
    Xxxl,
}

impl MediaQuery {
    pub(crate) fn write_selector(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            MediaQuery::Sm => write!(stream, "@media(min-width:640px)")?,
            MediaQuery::Md => write!(stream, "@media(min-width:768px)")?,
            MediaQuery::Lg => write!(stream, "@media(min-width:1024px)")?,
            MediaQuery::Xl => write!(stream, "@media(min-width:1280px)")?,
            MediaQuery::Xxl => write!(stream, "@media(min-width:1536px)")?,
            MediaQuery::Xxxl => write!(stream, "@media(min-width:1920px)")?,
        };

        Ok(())
    }

    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            MediaQuery::Sm => write!(stream, "sm")?,
            MediaQuery::Md => write!(stream, "md")?,
            MediaQuery::Lg => write!(stream, "lg")?,
            MediaQuery::Xl => write!(stream, "xl")?,
            MediaQuery::Xxl => write!(stream, "2xl")?,
            MediaQuery::Xxxl => write!(stream, "3xl")?,
        };

        Ok(())
    }
}
