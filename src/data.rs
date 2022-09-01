#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataCategory {
    Numerical,
    Categorical,
    Ordinal,
}