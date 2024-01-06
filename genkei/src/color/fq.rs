use std::hash::Hash;
use std::hash::Hasher;

/// Float quantized to 4 decimal places
#[derive(Debug, Copy, Clone)]
pub struct FloatQuantized(f64);

impl Eq for FloatQuantized {}

impl PartialEq for FloatQuantized {
    fn eq(&self, other: &Self) -> bool {
        self.as_i32() == other.as_i32()
    }
}

impl Ord for FloatQuantized {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_i32().cmp(&other.as_i32())
    }
}

impl PartialOrd for FloatQuantized {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for FloatQuantized {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_i32().hash(state);
    }
}

impl From<f64> for FloatQuantized {
    fn from(value: f64) -> Self {
        FloatQuantized(value)
    }
}

impl From<f32> for FloatQuantized {
    fn from(value: f32) -> Self {
        FloatQuantized(value as f64)
    }
}

impl std::fmt::Display for FloatQuantized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl FloatQuantized {
    pub fn as_i32(&self) -> i32 {
        (self.0 * 10000.0).round() as i32
    }

    pub fn as_f64(&self) -> f64 {
        self.as_i32() as f64 / 10000.0
    }
}
