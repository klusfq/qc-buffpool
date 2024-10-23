#[derive(Debug)]
pub struct QcBupoError;

impl std::fmt::Display for QcBupoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QcBupoError")
    }
}

impl std::error::Error for QcBupoError {}
