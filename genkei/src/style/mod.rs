mod attribute;
mod builder;
mod renderer;
mod styles;

pub use attribute::*;
pub use builder::*;
pub use renderer::*;
pub use styles::*;

use crate::Str;
use std::fmt::Write;

/// Represents a style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Style {
    Padding(Padding),
    Margin(Margin),
    Width(Width),
    MinWidth(MinWidth),
    Height(Height),
    MinHeight(MinHeight),
    Color(ColorStyle),
    Font(Font),
    TextAlign(TextAlign),
    Display(Display),
    Flex(Flex),
    Justify(Justify),
    Align(Align),
    Border(Border),
    Outline(Outline),
    Grid(Grid),
    Cursor(Cursor),
    BackdropFilter(BackdropFilter),
    Gap(Gap),
    CustomStyle(CustomStyleWrapper),

    State(State),
    MediaQuery(MediaQuery, Box<Style>),
    DataQuery(Str, Box<Style>),
}

impl Style {
    pub fn simplify(self) -> Self {
        match self {
            Style::State(x) => Style::State(x.simplify()),
            x => x,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub link: bool,
    pub visited: bool,
    pub focus: bool,
    pub focus_visible: bool,
    pub hover: bool,
    pub active: bool,
    pub backdrop: bool,
    pub inner: Box<Style>,
}

impl State {
    pub fn link(style: impl Into<Style>) -> Self {
        Self {
            link: true,
            visited: false,
            focus: false,
            focus_visible: false,
            hover: false,
            active: false,
            backdrop: false,
            inner: Box::new(style.into()),
        }
    }

    pub fn visited(style: impl Into<Style>) -> Self {
        Self {
            link: false,
            visited: true,
            focus: false,
            focus_visible: false,
            hover: false,
            active: false,
            backdrop: false,
            inner: Box::new(style.into()),
        }
    }

    pub fn focus(style: impl Into<Style>) -> Self {
        Self {
            link: false,
            visited: false,
            focus: true,
            focus_visible: false,
            hover: false,
            active: false,
            backdrop: false,
            inner: Box::new(style.into()),
        }
    }

    pub fn focus_visible(style: impl Into<Style>) -> Self {
        Self {
            link: false,
            visited: false,
            focus: false,
            focus_visible: true,
            hover: false,
            active: false,
            backdrop: false,
            inner: Box::new(style.into()),
        }
    }

    pub fn hover(style: impl Into<Style>) -> Self {
        Self {
            link: false,
            visited: false,
            focus: false,
            focus_visible: false,
            hover: true,
            active: false,
            backdrop: false,
            inner: Box::new(style.into()),
        }
    }

    pub fn active(style: impl Into<Style>) -> Self {
        Self {
            link: false,
            visited: false,
            focus: false,
            focus_visible: false,
            hover: false,
            active: true,
            backdrop: false,
            inner: Box::new(style.into()),
        }
    }

    pub fn backdrop(style: impl Into<Style>) -> Self {
        Self {
            link: false,
            visited: false,
            focus: false,
            focus_visible: false,
            hover: false,
            active: false,
            backdrop: true,
            inner: Box::new(style.into()),
        }
    }

    pub fn simplify(self) -> Self {
        let inner = self.inner.simplify();

        match inner {
            Style::State(x) => Self {
                link: self.link || x.link,
                visited: self.visited || x.visited,
                focus: self.focus || x.focus,
                focus_visible: self.focus_visible || x.focus_visible,
                hover: self.hover || x.hover,
                active: self.active || x.active,
                backdrop: self.backdrop || x.backdrop,
                inner: x.inner,
            },
            _ => Self {
                link: self.link,
                visited: self.visited,
                focus: self.focus,
                focus_visible: self.focus_visible,
                hover: self.hover,
                active: self.active,
                backdrop: self.backdrop,
                inner: Box::new(inner),
            },
        }
    }

    pub(crate) fn write_selector_prefix(&self, stream: &mut String) -> Result<(), StyleError> {
        if self.link {
            write!(stream, "link\\:")?;
        }
        if self.visited {
            write!(stream, "visited\\:")?;
        }
        if self.focus {
            write!(stream, "focus\\:")?;
        }
        if self.focus_visible {
            write!(stream, "focus-visible\\:")?;
        }
        if self.hover {
            write!(stream, "hover\\:")?;
        }
        if self.active {
            write!(stream, "active\\:")?;
        }
        if self.backdrop {
            write!(stream, "backdrop\\:")?;
        }

        Ok(())
    }

    pub(crate) fn write_selector_suffix(&self, stream: &mut String) -> Result<(), StyleError> {
        if self.link {
            write!(stream, ":link")?;
        }
        if self.visited {
            write!(stream, ":visited")?;
        }
        if self.focus {
            write!(stream, ":focus")?;
        }
        if self.focus_visible {
            write!(stream, ":focus-visible")?;
        }
        if self.hover {
            write!(stream, ":hover")?;
        }
        if self.active {
            write!(stream, ":active")?;
        }
        if self.backdrop {
            write!(stream, "::backdrop")?;
        }

        Ok(())
    }

    pub(crate) fn write_classname_prefix(&self, stream: &mut String) -> Result<(), StyleError> {
        if self.link {
            write!(stream, "link:")?;
        }
        if self.visited {
            write!(stream, "visited:")?;
        }
        if self.focus {
            write!(stream, "focus:")?;
        }
        if self.focus_visible {
            write!(stream, "focus-visible:")?;
        }
        if self.hover {
            write!(stream, "hover:")?;
        }
        if self.active {
            write!(stream, "active:")?;
        }
        if self.backdrop {
            write!(stream, "backdrop:")?;
        }

        Ok(())
    }
}

impl From<State> for Style {
    fn from(value: State) -> Self {
        Style::State(value)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare 'link'
        match self.link.cmp(&other.link) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'visited'
        match self.visited.cmp(&other.visited) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'focus'
        match self.focus.cmp(&other.focus) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'focus_visible'
        match self.focus_visible.cmp(&other.focus_visible) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'hover'
        match self.hover.cmp(&other.hover) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'active'
        match self.active.cmp(&other.active) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'backdrop'
        match self.backdrop.cmp(&other.backdrop) {
            std::cmp::Ordering::Equal => {}
            x => return x,
        }

        // Compare 'inner'
        self.inner.cmp(&other.inner)
    }
}

pub trait CustomStyle: std::fmt::Debug {
    /// Returns the class name for this style. This can be customized and doesn't have to
    /// be related to css. Must be unique between all styles.
    fn to_classname(&self, stream: &mut String);

    /// Returns the css statement for this style. The ending semicolon should not be included.
    fn to_css_statement(&self, stream: &mut String);

    /// Clone implementation for this style.
    fn clone_style(&self) -> Box<dyn CustomStyle + Send + Sync>;

    /// PartialEq implementation for this style.
    fn partial_eq_style(&self, other: &dyn CustomStyle) -> bool;

    /// PartialOrd implementation for this style.
    fn partial_cmp_style(&self, other: &dyn CustomStyle) -> Option<std::cmp::Ordering>;

    /// Ord implementation for this style.
    fn cmp_style(&self, other: &dyn CustomStyle) -> std::cmp::Ordering;

    /// Hash implementation for this style.
    fn hash_style(&self, state: &mut dyn std::hash::Hasher);
}

#[derive(Debug)]
pub struct CustomStyleWrapper(pub Box<dyn CustomStyle + Send + Sync>);

impl Clone for CustomStyleWrapper {
    fn clone(&self) -> Self {
        CustomStyleWrapper(self.0.clone_style())
    }
}

impl PartialEq for CustomStyleWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.partial_eq_style(&*other.0)
    }
}

impl Eq for CustomStyleWrapper {}

impl PartialOrd for CustomStyleWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomStyleWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp_style(&*other.0)
    }
}

impl std::hash::Hash for CustomStyleWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash_style(state)
    }
}
