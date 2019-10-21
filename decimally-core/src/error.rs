#[derive(Debug)]
pub enum DecimalStorageError {
    ExponentTooLarge,
    ExponentTooSmall,
    CoeffecientTooLarge,
}
