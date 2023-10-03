use super::{Style, Value};

pub trait AttributeExtension: Sized {
    // padding
    fn p(self, value: impl Into<Value>) -> Self {
        let value = value.into();
        self.px(value).py(value)
    }

    fn px(self, value: impl Into<Value>) -> Self {
        let value = value.into();
        self.pl(value).pr(value)
    }

    fn py(self, value: impl Into<Value>) -> Self {
        let value = value.into();
        self.pt(value).pb(value)
    }

    fn pt(self, value: impl Into<Value>) -> Self {
        self.apply_style(Style::Pt(value.into()))
    }

    fn pr(self, value: impl Into<Value>) -> Self {
        self.apply_style(Style::Pr(value.into()))
    }

    fn pb(self, value: impl Into<Value>) -> Self {
        self.apply_style(Style::Pb(value.into()))
    }

    fn pl(self, value: impl Into<Value>) -> Self {
        self.apply_style(Style::Pl(value.into()))
    }

    // styler
    fn apply_style(self, style: impl Into<Style>) -> Self;
}
