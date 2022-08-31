#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataCategory {
    Numerical,
    Categorical,
    Ordinal,
}