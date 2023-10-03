#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Integer(i32),
}

impl Into<Value> for i32 {
    fn into(self) -> Value {
        Value::Integer(self)
    }
}
