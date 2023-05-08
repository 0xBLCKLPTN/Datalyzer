
pub struct CsvNormalizer;
pub struct NewExcelNormalizer;
pub struct OldExcelNormalizer;
pub struct NormalizedData;

pub trait FromCSVNormalizer {
    fn convert() -> Self;
}


impl FromCSVNormalizer for CsvNormalizer {
    fn convert() -> Self {
        Self
    }
}