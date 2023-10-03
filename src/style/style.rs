use super::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Style {
    Pt(Value),
    Pr(Value),
    Pb(Value),
    Pl(Value),
}
